use std::env::var;

fn main() {
    println!("cargo:rustc-check-cfg=cfg(uart, values(\"16550\",\"pl011\"))");
    println!(
        "cargo:rustc-cfg=uart=\"{}\"",
        match var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
            "aarch64" => "pl011",
            "x86_64" => "16550",
            "riscv64" => "16550",
            _ => unimplemented!(),
        }
    );
}
