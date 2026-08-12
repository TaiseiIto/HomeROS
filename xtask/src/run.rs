use {
    crate::{
        command::run,
        git::product,
        product::{Arch, build},
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
        let Self { arch } = self;
        match arch {
            Arch::RiscV64 => format!("-kernel {}", arch.boot_destination().to_str().unwrap()),
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
        let Self { arch } = self;
        match arch {
            Arch::X64 => "-serial file:com2.log",
            _ => "",
        }
    }

    fn cpu(&self) -> &str {
        let Self { arch } = self;
        match arch {
            Arch::Aarch64 => "-cpu max",
            _ => "",
        }
    }

    fn debug(&self) -> String {
        let Self { arch } = self;
        match arch {
            Arch::Aarch64 => format!("-serial file:{}", Self::DEBUG),
            Arch::X64 => format!(
                "-chardev file,id=debug,path={} -device isa-debugcon,iobase=0x402,chardev=debug",
                Self::DEBUG
            ),
            _ => String::new(),
        }
    }

    fn display(&self) -> &str {
        let Self { arch } = self;
        match arch {
            Arch::Aarch64 | Arch::RiscV64 => "-device ramfb",
            _ => "",
        }
    }

    fn drive(&self) -> String {
        let Self { arch } = self;
        match arch {
            Arch::Aarch64 => format!(
                "-drive file=fat:rw:{},format=raw,id={},if=none -device virtio-blk-device,drive={},bootindex=1",
                arch.destination().to_str().unwrap(),
                product(),
                product()
            ),
            Arch::RiscV64 => format!(
                "-drive format=raw,file=fat:rw:{}",
                arch.destination().to_str().unwrap(),
            ),
            Arch::X64 => format!(
                "-drive file=fat:rw:{},format=raw,id={},if=none -device ide-hd,drive={},bootindex=1",
                arch.destination().to_str().unwrap(),
                product(),
                product()
            ),
        }
    }

    fn firmware(&self) -> String {
        let Self { arch } = self;
        match arch {
            Arch::Aarch64 => {
                format!("-drive if=pflash,format=raw,unit=0,file={},readonly=on", arch.boot_destination().to_str().unwrap())
            }
            Arch::RiscV64 => "-bios default".to_string(),
            Arch::X64 => {
                "-drive file=../edk2/Build/OvmfX64/DEBUG_GCC/FV/OVMF_CODE.fd,format=raw,if=pflash,readonly=on -drive file=../edk2/Build/OvmfX64/DEBUG_GCC/FV/OVMF_VARS.fd,format=raw,if=pflash".to_string()
            }
        }
    }

    fn machine(&self) -> &str {
        let Self { arch } = self;
        match arch {
            Arch::Aarch64 | Arch::RiscV64 => "-machine virt",
            Arch::X64 => "",
        }
    }

    fn qemu(&self) -> &str {
        let Self { arch } = self;
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
        let Self { arch } = self;
        let source: PathBuf = PathBuf::from(".docker/tmux/run");
        run(&format!(
            "QEMU_ARCH={} tmux new-session ; source-file {}",
            arch,
            source.to_str().unwrap()
        ));
    }
}

impl Display for Command {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        let Self { arch } = self;
        write!(formatter, "{}", arch)
    }
}

impl From<Args> for Command {
    fn from(mut args: Args) -> Self {
        let arch: Arch = args.next().unwrap().as_str().into();
        Self { arch }
    }
}
