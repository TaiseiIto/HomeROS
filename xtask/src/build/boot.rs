use {
    crate::command::run,
    std::path::{Path, PathBuf},
};

pub fn all(destination: &Path) {
    let package: &str = "boot";
    aarch64(package, destination);
    riscv64(package, destination);
    x64(package, destination);
}

fn aarch64(package: &str, destination: &Path) {
    let target: &str = "aarch64-unknown-uefi";
    let source: PathBuf = PathBuf::from(&format!("target/{}/debug/{}.efi", target, package));
    let mut destination: PathBuf = destination.to_path_buf();
    destination.push("efi");
    destination.push("boot");
    destination.push("bootaa64.efi");
    run(&format!(
        "cargo build --package {} --target {}",
        package, target
    ));
    run(&format!(
        "mkdir -p {}",
        destination.parent().unwrap().to_str().unwrap()
    ));
    run(&format!(
        "cp {} {}",
        source.to_str().unwrap(),
        destination.to_str().unwrap()
    ));
}

fn riscv64(package: &str, destination: &Path) {
    let target: &str = "riscv64gc-unknown-none-elf";
    let source: PathBuf = PathBuf::from(&format!("target/{}/debug/{}", target, package));
    let mut destination: PathBuf = destination.to_path_buf();
    destination.push("boot");
    destination.push("riscv64.elf");
    run(&format!(
        "RUSTFLAGS=\"-C link-arg=boot/firmware/open_sbi/link.ld\" cargo build --package {} --target {}",
        package, target
    ));
    run(&format!(
        "mkdir -p {}",
        destination.parent().unwrap().to_str().unwrap()
    ));
    run(&format!(
        "cp {} {}",
        source.to_str().unwrap(),
        destination.to_str().unwrap()
    ));
}

fn x64(package: &str, destination: &Path) {
    let target: &str = "x86_64-unknown-uefi";
    let source: PathBuf = PathBuf::from(&format!("target/{}/debug/{}.efi", target, package));
    let mut destination: PathBuf = destination.to_path_buf();
    destination.push("efi");
    destination.push("boot");
    destination.push("bootx64.efi");
    run(&format!(
        "cargo build --package {} --target {}",
        package, target
    ));
    run(&format!(
        "mkdir -p {}",
        destination.parent().unwrap().to_str().unwrap()
    ));
    run(&format!(
        "cp {} {}",
        source.to_str().unwrap(),
        destination.to_str().unwrap()
    ));
}
