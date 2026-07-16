mod command;
mod docker;
mod environment;
mod git;
mod time;

fn main() {
    environment::remove();
    environment::build();
}
