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
    const COM1: &str = "-serial file:com1.log";
    const COM2: &str = "-chardev stdio,id=com2,mux=on,logfile=com2.log -serial chardev:com2";
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

    fn command(&self) -> String {
        [
            self.qemu(),
            self.cpu(),
            self.machine(),
            self.firmware(),
            &self.drive(),
            Self::COM1,
            Self::COM2,
            Self::LOG,
            Self::MEMORY,
            Self::REBOOT,
            Self::VNC,
        ]
        .join(" ")
    }

    fn cpu(&self) -> &str {
        let Self { arch } = self;
        match arch {
            Arch::Aarch64 => "-cpu max",
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
            Arch::RiscV64 => unimplemented!(),
            Arch::X64 => format!(
                "-drive file=fat:rw:{},format=raw,id={},if=none -device ide-hd,drive={},bootindex=1",
                arch.destination().to_str().unwrap(),
                product(),
                product()
            ),
        }
    }

    fn firmware(&self) -> &str {
        let Self { arch } = self;
        match arch {
            Arch::Aarch64 => {
                "-drive file=../edk2/Build/ArmVirtQemu-AArch64/DEBUG_GCC/FV/QEMU_EFI.fd,format=raw,if=pflash,readonly=on -drive file=../edk2/Build/ArmVirtQemu-AArch64/DEBUG_GCC/FV/QEMU_VARS.fd,format=raw,if=pflash,readonly=on"
            }
            Arch::RiscV64 => "",
            Arch::X64 => {
                "-drive file=../edk2/Build/OvmfX64/DEBUG_GCC/FV/OVMF_CODE.fd,format=raw,if=pflash,readonly=on -drive file=../edk2/Build/OvmfX64/DEBUG_GCC/FV/OVMF_VARS.fd,format=raw,if=pflash,readonly=on"
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
