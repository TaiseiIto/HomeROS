use {
    clap::{Parser, Subcommand},
    xtask::environment,
};

fn main() {
    match Arguments::parse().command {
        Command::Build => {
            environment::attach();
        }
        Command::Delete => environment::remove(),
        Command::Privilege { gpg_key, ssh_key } => {
            environment::privilege(
                std::path::Path::new(&gpg_key),
                std::path::Path::new(&ssh_key),
            );
        }
        Command::Rebuild => {
            environment::remove();
            environment::attach();
        }
    }
}

#[derive(Parser)]
struct Arguments {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Build,
    Delete,
    Privilege {
        #[arg(long, short)]
        gpg_key: String,
        #[arg(long, short)]
        ssh_key: String,
    },
    Rebuild,
}
