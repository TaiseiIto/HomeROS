use {
    crate::{command::run, product::Arch},
    std::{env::Args, path::PathBuf},
};

pub struct Command(Arch);

impl Command {
    pub fn arch(&self) -> &Arch {
        &self.0
    }

    pub fn run(self) {
        let source: PathBuf = PathBuf::from(".docker/tmux/dbg");
        run(&format!(
            "ARCH={} VERSION=debug tmux new-session ; source-file {}",
            self.arch(),
            source.to_str().unwrap()
        ));
    }

    fn new(arch: Arch) -> Self {
        Self(arch)
    }
}

impl From<Args> for Command {
    fn from(mut args: Args) -> Self {
        let mut arch: Option<Arch> = None;
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--arch" => arch = Some(args.next().unwrap().as_str().into()),
                arg => unreachable!("arg = {}", arg),
            }
        }
        Self::new(arch.unwrap())
    }
}
