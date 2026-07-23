use {crate::command::run, std::env::Args};

pub struct Command {
    arch: Arch,
}

impl Command {
    pub fn run(self) {
        let Self { arch } = self;
        let qemu: &str = arch.qemu();
        run(qemu);
    }
}

impl From<Args> for Command {
    fn from(mut args: Args) -> Self {
        let mut arch: Option<Arch> = None;
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--arch" => arch = Some(args.next().unwrap().as_str().into()),
                arg => panic!("arg = {}", arg),
            }
        }
        let arch: Arch = arch.unwrap();
        Self { arch }
    }
}

enum Arch {
    Aarch64,
    RiscV64,
    X64,
}

impl Arch {
    fn qemu(&self) -> &str {
        match self {
            Self::Aarch64 => "qemu-system-aarch64",
            Self::RiscV64 => "qemu-system-riscv64",
            Self::X64 => "qemu-system-x86_64",
        }
    }
}

impl From<&str> for Arch {
    fn from(arch: &str) -> Self {
        match arch {
            "aarch64" => Self::Aarch64,
            "riscv64" => Self::RiscV64,
            "x64" => Self::X64,
            _ => unimplemented!(),
        }
    }
}
