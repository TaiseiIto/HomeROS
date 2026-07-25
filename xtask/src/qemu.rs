use {super::run::Arch, crate::command::run, std::env::Args};

pub struct Command {
    arch: Arch,
}

impl Command {
    pub fn run(self) {
        run(&format!("{} {} -vnc :0", self.qemu(), self.machine()));
    }

    fn machine(&self) -> &str {
        let Self { arch } = self;
        match arch {
            Arch::Aarch64 => "-machine virt",
            Arch::RiscV64 => "",
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
