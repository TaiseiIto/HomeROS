use xtask::{build, environment, format, git, lint};

fn main() {
    match std::env::args().into() {
        Command::Build => {
            build::all();
        }
        Command::Environment(Environment::Build) => {
            environment::attach();
        }
        Command::Environment(Environment::Delete) => environment::remove(),
        Command::Environment(Environment::Privilege { gpg_key, ssh_key }) => {
            environment::privilege(&gpg_key, &ssh_key);
        }
        Command::Environment(Environment::Rebuild) => {
            environment::remove();
            environment::attach();
        }
        Command::Lint => {
            lint::all();
        }
        Command::PreCommit => {
            lint::all();
            format::all();
            git::add_rust_sources();
        }
        Command::Run => unimplemented!(),
    }
}

enum Command {
    Build,
    Environment(Environment),
    Lint,
    PreCommit,
    Run,
}

impl From<std::env::Args> for Command {
    fn from(mut args: std::env::Args) -> Self {
        args.next();
        match args.next().unwrap().as_str() {
            "build" => Self::Build,
            "environment" => Self::Environment(args.into()),
            "lint" => Self::Lint,
            "precommit" => Self::PreCommit,
            "run" => Self::Run,
            arg => panic!("arg = {}", arg),
        }
    }
}

enum Environment {
    Build,
    Delete,
    Privilege {
        gpg_key: std::path::PathBuf,
        ssh_key: std::path::PathBuf,
    },
    Rebuild,
}

impl From<std::env::Args> for Environment {
    fn from(mut args: std::env::Args) -> Self {
        match args.next().as_deref() {
            None => Self::Build,
            Some("delete") => Self::Delete,
            Some("privilege") => {
                let mut gpg_key: Option<std::path::PathBuf> = None;
                let mut ssh_key: Option<std::path::PathBuf> = None;
                while let Some(arg) = args.next() {
                    match arg.as_str() {
                        "--gpg-key" => gpg_key = Some(args.next().unwrap().into()),
                        "--ssh-key" => ssh_key = Some(args.next().unwrap().into()),
                        arg => panic!("arg = {}", arg),
                    }
                }
                let gpg_key: std::path::PathBuf = gpg_key.unwrap();
                let ssh_key: std::path::PathBuf = ssh_key.unwrap();
                Self::Privilege { gpg_key, ssh_key }
            }
            Some("rebuild") => Self::Rebuild,
            Some(arg) => panic!("arg = {}", arg),
        }
    }
}
