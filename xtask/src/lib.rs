use std::env::{Args, args};

pub mod build;
mod command;
pub mod disassemble;
mod docker;
pub mod environment;
pub mod format;
pub mod git;
pub mod lint;
mod time;

pub enum Command {
    Build,
    Disassemble(disassemble::Command),
    Environment(environment::Command),
    Lint,
    PreCommit,
    Run,
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
            Self::Disassemble(disassemble) => {
                build::all();
                disassemble.run();
            }
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

impl From<Args> for Command {
    fn from(mut args: Args) -> Self {
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
