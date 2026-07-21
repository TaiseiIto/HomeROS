use {std::env::args, xtask::Command};

fn main() {
    let command: Command = args().into();
    command.run();
}
