use crate::command::run;

pub fn all() {
    aarch64();
    riscv64();
    x64();
}

fn aarch64() {
    run("cargo clippy --package boot --target aarch64-unknown-uefi");
}

fn riscv64() {
    run(
        "RUSTFLAGS=\"-C link-arg=boot/firmware/open_sbi/link.ld\" cargo clippy --package boot --target riscv64gc-unknown-none-elf",
    );
}

fn x64() {
    run("cargo clippy --package boot --target x86_64-unknown-uefi");
}
