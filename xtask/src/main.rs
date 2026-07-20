use xtask::{build, disassemble, environment, format, git, lint};

fn main() {
    let command: Command = std::env::args().into();
    command.run();
}

enum Command {
    Build,
    Disassemble(disassemble::Command),
    Environment(environment::Command),
    Lint,
    PreCommit,
    Run,
}

impl Command {
    fn run(self) {
        match std::env::args().into() {
            Self::Build => build::all(),
            Self::Disassemble(disassemble) => disassemble.run(),
            Self::Environment(environment) => environment.run(),
            Self::Lint => lint::all(),
            Self::PreCommit => {
                lint::all();
                format::all();
                git::add_rust_sources();
            }
            Self::Run => unimplemented!(),
        }
    }
}

impl From<std::env::Args> for Command {
    fn from(mut args: std::env::Args) -> Self {
        args.next();
        match args.next().unwrap().as_str() {
            "build" => Self::Build,
            "disassemble" => Self::Disassemble(args.into()),
            "environment" => Self::Environment(args.into()),
            "lint" => Self::Lint,
            "precommit" => Self::PreCommit,
            "run" => Self::Run,
            arg => panic!("arg = {}", arg),
        }
    }
}
