use std::env::{Args, args};

mod command;
mod disassemble;
mod docker;
mod environment;
mod format;
mod git;
mod lint;
mod product;
mod qemu;
mod run;
mod time;

pub use {docker::in_container, format::format, lint::lint};

pub enum Command {
    Build,
    Disassemble(disassemble::Command),
    Environment(environment::Command),
    Lint,
    PreCommit,
    Qemu(qemu::Command),
    Run(run::Command),
}

impl Command {
    pub fn run(self) {
        match args().into() {
            Self::Build => {
                if in_container() {
                    product::build()
                } else {
                    environment::build_in_container();
                }
            }
            Self::Disassemble(command) => {
                product::build();
                command.run();
            }
            Self::Environment(command) => command.run(),
            Self::Lint => lint(),
            Self::PreCommit => {
                git::add_rust_sources();
                lint();
                format();
                git::add_rust_sources();
            }
            Self::Qemu(command) => command.run(),
            Self::Run(command) => {
                if in_container() {
                    product::build();
                    command.run();
                } else {
                    environment::run_in_container(command);
                }
            }
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
            "qemu" => Self::Qemu(args.into()),
            "run" => Self::Run(args.into()),
            arg => panic!("arg = {}", arg),
        }
    }
}
