use {super::run::Arch, crate::command::run, std::env::Args};

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
        run(&self.command());
    }

    fn command(&self) -> String {
        [
            self.qemu(),
            self.machine(),
            self.firmware(),
            Self::COM1,
            Self::COM2,
            Self::LOG,
            Self::MEMORY,
            Self::REBOOT,
            Self::VNC,
        ]
        .join(" ")
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
            Arch::Aarch64 => "-machine virt",
            Arch::RiscV64 => "-machine virt",
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
}

impl From<Args> for Command {
    fn from(mut args: Args) -> Self {
        let arch: Arch = args.next().unwrap().as_str().into();
        Self { arch }
    }
}
