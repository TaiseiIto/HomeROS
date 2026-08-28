use {
    crate::{
        command::run,
        git::product,
        product::{Arch, Tree, Version, build},
        tmux,
    },
    std::{
        env::Args,
        fmt::{Display, Formatter, Result},
        path::PathBuf,
    },
};

pub struct Command {
    arch: Arch,
    version: Version,
}

impl Command {
    const COM1: &str = "-chardev stdio,id=com1,mux=on,logfile=com1.log -serial chardev:com1";
    const DEBUG: &str = "debug.log";
    const LOG: &str = "-d int,cpu_reset -D qemu.log";
    const MEMORY: &str = "-m 1G";
    const REBOOT: &str = "--no-reboot";
    const VNC: &str = "-vnc :0";

    pub fn run(self) {
        if tmux::runs() {
            self.run_inside_tmux();
        } else {
            self.run_outside_tmux();
        }
    }

    fn boot(&self) -> String {
        let Self { arch, version } = self;
        match arch {
            Arch::RiscV64 => format!(
                "-kernel {}",
                arch.boot_destination(version).to_str().unwrap()
            ),
            _ => String::new(),
        }
    }

    fn command(&self) -> String {
        [
            self.qemu(),
            &self.boot(),
            Self::COM1,
            self.com2(),
            self.cpu(),
            &self.debug(),
            self.display(),
            &self.drive(),
            &self.firmware(),
            Self::LOG,
            self.machine(),
            Self::MEMORY,
            Self::REBOOT,
            Self::VNC,
        ]
        .join(" ")
    }

    fn com2(&self) -> &str {
        let Self { arch, version: _ } = self;
        match arch {
            Arch::X64 => "-serial file:com2.log",
            _ => "",
        }
    }

    fn cpu(&self) -> &str {
        let Self { arch, version: _ } = self;
        match arch {
            Arch::Aarch64 => "-cpu max",
            _ => "",
        }
    }

    fn debug(&self) -> String {
        let Self { arch, version: _ } = self;
        match arch {
            Arch::Aarch64 => format!("-serial file:{}", Self::DEBUG),
            Arch::X64 => format!(
                "-chardev file,id=debug,path={} -device isa-debugcon,iobase=0x402,chardev=debug",
                Self::DEBUG
            ),
            _ => String::new(),
        }
    }

    fn destination(&self) -> PathBuf {
        let tree: Tree = self.into();
        tree.destination()
    }

    fn display(&self) -> &str {
        let Self { arch, version: _ } = self;
        match arch {
            Arch::Aarch64 | Arch::RiscV64 => "-device ramfb",
            _ => "",
        }
    }

    fn drive(&self) -> String {
        match self.arch {
            Arch::Aarch64 => format!(
                "-drive file=fat:rw:{},format=raw,id={},if=none -device virtio-blk-device,drive={},bootindex=1",
                self.destination().to_str().unwrap(),
                product(),
                product()
            ),
            Arch::RiscV64 => format!(
                "-drive format=raw,file=fat:rw:{}",
                self.destination().to_str().unwrap(),
            ),
            Arch::X64 => format!(
                "-drive file=fat:rw:{},format=raw,id={},if=none -device ide-hd,drive={},bootindex=1",
                self.destination().to_str().unwrap(),
                product(),
                product()
            ),
        }
    }

    fn firmware(&self) -> String {
        let Self { arch, version } = self;
        match arch {
            Arch::Aarch64 => {
                format!("-drive if=pflash,format=raw,unit=0,file={},readonly=on", arch.boot_destination(version).to_str().unwrap())
            }
            Arch::RiscV64 => "-bios default".to_string(),
            Arch::X64 => {
                "-drive file=../edk2/Build/OvmfX64/DEBUG_GCC/FV/OVMF_CODE.fd,format=raw,if=pflash,readonly=on -drive file=../edk2/Build/OvmfX64/DEBUG_GCC/FV/OVMF_VARS.fd,format=raw,if=pflash".to_string()
            }
        }
    }

    fn machine(&self) -> &str {
        let Self { arch, version: _ } = self;
        match arch {
            Arch::Aarch64 => "-machine virt,secure=on",
            Arch::RiscV64 => "-machine virt",
            Arch::X64 => "",
        }
    }

    fn new(arch: Arch, version: Version) -> Self {
        Self { arch, version }
    }

    fn qemu(&self) -> &str {
        let Self { arch, version: _ } = self;
        match arch {
            Arch::Aarch64 => "qemu-system-aarch64",
            Arch::RiscV64 => "qemu-system-riscv64",
            Arch::X64 => "qemu-system-x86_64",
        }
    }

    fn run_inside_tmux(self) {
        run(&self.command());
    }

    fn run_outside_tmux(self) {
        build();
        let Self { arch, version } = self;
        let source: PathBuf = PathBuf::from(".docker/tmux/run");
        run(&format!(
            "ARCH={} VERSION={} tmux new-session ; source-file {}",
            arch,
            version,
            source.to_str().unwrap()
        ));
    }
}

impl From<&Command> for Tree {
    fn from(command: &Command) -> Self {
        let Command { arch, version } = command;
        Self::new(arch.clone(), version.clone())
    }
}

impl Display for Command {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        let Self { arch, version } = self;
        write!(formatter, "--arch {} --version {}", arch, version)
    }
}

impl From<Args> for Command {
    fn from(mut args: Args) -> Self {
        let mut arch: Option<Arch> = None;
        let mut version: Option<Version> = None;
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--arch" => arch = Some(args.next().unwrap().as_str().into()),
                "--version" => version = Some(args.next().unwrap().as_str().into()),
                arg => unreachable!("arg = {}", arg),
            }
        }
        Self::new(arch.unwrap(), version.unwrap())
    }
}
