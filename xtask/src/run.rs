use {
    crate::{command::run, product::Arch},
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
