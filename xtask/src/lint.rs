use crate::command;

pub fn all() {
    boot();
    xtask();
}

fn boot() {
    command::run("cargo clippy -p boot --target x86_64-unknown-uefi --all-features -- -D warnings")
}

fn xtask() {
    command::run("cargo clippy -p xtask --all-features -- -D warnings")
}
