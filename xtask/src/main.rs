use xtask::environment;

fn main() {
    environment::remove();
    environment::build();
}
