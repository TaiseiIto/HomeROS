use crate::command;

pub fn aarch64() {
    command::run("cargo build --package boot --target aarch64-unknown-uefi");
}

pub fn x86_64() {
    command::run("cargo build --package boot --target x86_64-unknown-uefi");
}
