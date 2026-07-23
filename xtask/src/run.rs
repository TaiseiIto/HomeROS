use {
    crate::command::run,
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
    pub fn run(self) {
        let Self { arch } = self;
        let source: PathBuf = PathBuf::from(".docker/.tmux/run");
        run(&format!(
            "QEMU_ARCH={} tmux new-session ; source-file {}",
            arch,
            source.to_str().unwrap()
        ));
    }
}

impl From<Args> for Command {
    fn from(mut args: Args) -> Self {
        let arch: Arch = args.next().unwrap().as_str().into();
        Self { arch }
    }
}

enum Arch {
    Aarch64,
    RiscV64,
    X64,
}

impl Display for Arch {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        let string: String = self.into();
        write!(formatter, "{}", string)
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

impl From<&Arch> for String {
    fn from(arch: &Arch) -> String {
        match arch {
            Arch::Aarch64 => "aarch64",
            Arch::RiscV64 => "riscv64",
            Arch::X64 => "x64",
        }
        .to_string()
    }
}
