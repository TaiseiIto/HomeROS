use {
    clap::{Parser, Subcommand},
    xtask::environment,
};

#[derive(Parser)]
struct Arguments {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Build,
    Delete,
    Rebuild,
}

fn main() {
    match Arguments::parse().command {
        Command::Build => environment::build(),
        Command::Delete => environment::remove(),
        Command::Rebuild => {
            environment::remove();
            environment::build();
        }
    }
}
