use crate::command::run;

pub fn all() {
    let package: &str = "boot";
    aarch64(package);
    riscv64(package);
    x64(package);
}

fn aarch64(package: &str) {
    let target: &str = "aarch64-unknown-uefi";
    run(&format!(
        "cargo build --package {} --target {}",
        package, target
    ));
}

fn riscv64(package: &str) {
    let target: &str = "riscv64gc-unknown-none-elf";
    run(&format!(
        "RUSTFLAGS=\"-C link-arg=boot/firmware/open_sbi/link.ld\" cargo build --package {} --target {}",
        package, target
    ));
}

fn x64(package: &str) {
    let target: &str = "x86_64-unknown-uefi";
    run(&format!(
        "cargo build --package {} --target {}",
        package, target
    ));
}
