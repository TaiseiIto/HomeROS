use xtask::environment;

fn main() {
    match std::env::args().into() {
        Command::Build => {
            environment::attach();
        }
        Command::Delete => environment::remove(),
        Command::Privilege { gpg_key, ssh_key } => {
            environment::privilege(&gpg_key, &ssh_key);
        }
        Command::Rebuild => {
            environment::remove();
            environment::attach();
        }
    }
}

enum Command {
    Build,
    Delete,
    Privilege {
        gpg_key: std::path::PathBuf,
        ssh_key: std::path::PathBuf,
    },
    Rebuild,
}

impl From<std::env::Args> for Command {
    fn from(mut args: std::env::Args) -> Self {
        match args.next().unwrap().as_str() {
            "build" => Self::Build,
            "delete" => Self::Delete,
            "privilege" => {
                let mut gpg_key: Option<std::path::PathBuf> = None;
                let mut ssh_key: Option<std::path::PathBuf> = None;
                while let Some(arg) = args.next() {
                    match arg.as_str() {
                        "--gpg-key" => gpg_key = Some(args.next().unwrap().into()),
                        "--ssh-key" => ssh_key = Some(args.next().unwrap().into()),
                        _ => unreachable!(),
                    }
                }
                let gpg_key: std::path::PathBuf = gpg_key.unwrap();
                let ssh_key: std::path::PathBuf = ssh_key.unwrap();
                Self::Privilege { gpg_key, ssh_key }
            }
            "rebuild" => Self::Rebuild,
            _ => unreachable!(),
        }
    }
}
