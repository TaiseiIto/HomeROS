use xtask::Command;

fn main() {
    let command: Command = std::env::args().into();
    command.run();
}
