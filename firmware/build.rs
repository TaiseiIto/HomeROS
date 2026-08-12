use std::env::var;

fn main() {
    println!("cargo:rustc-check-cfg=cfg(firmware, values(\"open_sbi\",\"tfa\",\"uefi\"))");
    println!(
        "cargo:rustc-cfg=firmware=\"{}\"",
        match var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
            "aarch64" => "tfa",
            "x86_64" => "uefi",
            "riscv64" => "open_sbi",
            _ => unimplemented!(),
        }
    );
}
