mod command;
mod develop;
mod disassemble;
mod docker;
mod firmware;
mod format;
mod git;
mod install;
mod lint;
mod product;
mod run;
mod test;
mod time;
mod tmux;

use std::env::Args;

pub use {docker::in_container, format::format, lint::lint, test::test};

pub enum Command {
    Build,
    Develop(develop::Command),
    Disassemble(disassemble::Command),
    Install(install::Command),
    Lint,
    PreCommit,
    Run(run::Command),
    Test,
}

impl Command {
    pub fn run(self) {
        match self {
            Self::Build => {
                if in_container() {
                    product::build()
                } else {
                    develop::build_in_container();
                }
            }
            Self::Develop(command) => command.run(),
            Self::Disassemble(command) => {
                product::build();
                command.run();
            }
            Self::Install(command) => {
                product::build();
                command.run();
            }
            Self::Lint => lint(),
            Self::PreCommit => {
                git::add_rust_sources();
                Self::Test.run();
                Self::Build.run();
                Self::Lint.run();
                format();
                git::add_rust_sources();
            }
            Self::Run(command) => {
                if in_container() {
                    command.run();
                } else {
                    develop::run_in_container(command);
                }
            }
            Self::Test => test(),
        }
    }
}

impl From<Args> for Command {
    fn from(mut args: Args) -> Self {
        args.next();
        match args.next().unwrap().as_str() {
            "build" => Self::Build,
            "develop" => Self::Develop(args.into()),
            "disassemble" => Self::Disassemble(args.into()),
            "install" => Self::Install(args.into()),
            "lint" => Self::Lint,
            "precommit" => Self::PreCommit,
            "run" => Self::Run(args.into()),
            "test" => Self::Test,
            arg => panic!("arg = {}", arg),
        }
    }
}
