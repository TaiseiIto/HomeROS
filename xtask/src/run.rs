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
        match self.arch {
            Arch::RiscV64 => format!("-kernel {}", self.boot_destination().to_str().unwrap()),
            _ => String::new(),
        }
    }

    fn boot_destination(&self) -> PathBuf {
        let tree: Tree = self.into();
        tree.boot_destination()
    }

    fn command(&self) -> String {
        [
            self.qemu(),
            &self.boot(),
            &self.com1(),
            &self.com2(),
            self.cpu(),
            &self.debug(),
            self.display(),
            &self.drive(),
            &self.firmware(),
            &self.log(),
            self.machine(),
            Self::MEMORY,
            Self::REBOOT,
            Self::VNC,
        ]
        .join(" ")
    }

    fn com1(&self) -> String {
        let Self { arch, version: _ } = self;
        match arch {
            Arch::X64 => format!("-serial file:{}", self.com1log().to_str().unwrap()),
            _ => format!(
                "-chardev stdio,id=com1,mux=on,logfile={} -serial chardev:com1",
                self.com1log().to_str().unwrap()
            ),
        }
    }

    fn com1log(&self) -> PathBuf {
        let mut com1log: PathBuf = self.log_directory();
        com1log.push("com1.log");
        com1log
    }

    fn com2(&self) -> String {
        let Self { arch, version: _ } = self;
        match arch {
            Arch::X64 => format!(
                "-chardev stdio,id=com2,mux=on,logfile={} -serial chardev:com2",
                self.com2log().to_str().unwrap()
            ),
            _ => String::new(),
        }
    }

    fn com2log(&self) -> PathBuf {
        let mut com2log: PathBuf = self.log_directory();
        com2log.push("com2.log");
        com2log
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
            Arch::Aarch64 => format!("-serial file:{}", self.debug_log().to_str().unwrap()),
            Arch::X64 => format!(
                "-chardev file,id=debug,path={} -device isa-debugcon,iobase=0x402,chardev=debug",
                self.debug_log().to_str().unwrap(),
            ),
            _ => String::new(),
        }
    }

    fn debug_log(&self) -> PathBuf {
        let mut debug_log: PathBuf = self.log_directory();
        debug_log.push("debug.log");
        debug_log
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
        match self.arch {
            Arch::Aarch64 => {
                format!("-drive if=pflash,format=raw,unit=0,file={},readonly=on", self.boot_destination().to_str().unwrap())
            }
            Arch::RiscV64 => "-bios default".to_string(),
            Arch::X64 => {
                "-drive file=../edk2/Build/OvmfX64/DEBUG_GCC/FV/OVMF_CODE.fd,format=raw,if=pflash,readonly=on -drive file=../edk2/Build/OvmfX64/DEBUG_GCC/FV/OVMF_VARS.fd,format=raw,if=pflash".to_string()
            }
        }
    }

    fn log(&self) -> String {
        format!("-d int,cpu_reset -D {}", self.qemu_log().to_str().unwrap())
    }

    fn log_directory(&self) -> PathBuf {
        let mut log_directory: PathBuf = PathBuf::from("log");
        log_directory.push(match self.arch {
            Arch::Aarch64 => "aarch64",
            Arch::RiscV64 => "riscv64",
            Arch::X64 => "x64",
        });
        log_directory
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

    fn qemu_log(&self) -> PathBuf {
        let mut qemu_log: PathBuf = self.log_directory();
        qemu_log.push("qemu.log");
        qemu_log
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
