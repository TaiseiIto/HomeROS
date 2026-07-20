use crate::command;

pub fn all() {
    aarch64();
    riscv64();
    x86_64();
}

fn aarch64() {
    command::run("cargo build --package boot --target aarch64-unknown-uefi");
}

fn riscv64() {
    command::run(
        "RUSTFLAGS=\"-C link-arg=boot/firmware/open_sbi/link.ld\" cargo build --package boot --target riscv64gc-unknown-none-elf",
    );
}

fn x86_64() {
    command::run("cargo build --package boot --target x86_64-unknown-uefi");
}
