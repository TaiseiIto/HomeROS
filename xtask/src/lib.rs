use std::env::{Args, args};

pub mod build;
mod command;
pub mod disassemble;
mod docker;
pub mod environment;
pub mod format;
pub mod git;
pub mod lint;
pub mod run;
mod time;

pub enum Command {
    Build,
    Disassemble(disassemble::Command),
    Environment(environment::Command),
    Lint,
    PreCommit,
    Run(run::Command),
}

impl Command {
    pub fn run(self) {
        match args().into() {
            Self::Build => {
                if docker::is_installed() {
                    environment::execute("cargo xtask build");
                } else {
                    build::all()
                }
            }
            Self::Disassemble(command) => {
                build::all();
                command.run();
            }
            Self::Environment(command) => command.run(),
            Self::Lint => lint::all(),
            Self::PreCommit => {
                lint::all();
                format::all();
                git::add_rust_sources();
            }
            Self::Run(command) => command.run(),
        }
    }
}

impl From<Args> for Command {
    fn from(mut args: Args) -> Self {
        args.next();
        match args.next().unwrap().as_str() {
            "build" => Self::Build,
            "disassemble" => Self::Disassemble(args.into()),
            "environment" => Self::Environment(args.into()),
            "lint" => Self::Lint,
            "precommit" => Self::PreCommit,
            "run" => Self::Run(args.into()),
            arg => panic!("arg = {}", arg),
        }
    }
}
