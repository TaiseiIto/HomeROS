use std::env::var;

fn main() {
    println!("cargo:rustc-check-cfg=cfg(firmware, values(\"sbi\",\"tfa\",\"uefi\"))");
    println!(
        "cargo:rustc-cfg=firmware=\"{}\"",
        match var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
            "aarch64" => "tfa",
            "x86_64" => "uefi",
            "riscv64" => "sbi",
            _ => unimplemented!(),
        }
    );
    println!("cargo:rustc-check-cfg=cfg(has_device_tree)");
    println!("cargo:rustc-check-cfg=cfg(start_with_assembly)");
    println!("cargo:rustc-check-cfg=cfg(use_temporary_memory_allocator)");
    if matches!(
        var("CARGO_CFG_TARGET_ARCH").unwrap().as_str(),
        "aarch64" | "riscv64"
    ) {
        println!("cargo:rustc-cfg=has_device_tree");
        println!("cargo:rustc-cfg=start_with_assembly");
        println!("cargo:rustc-cfg=use_temporary_memory_allocator");
    }
}
