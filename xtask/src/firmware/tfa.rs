use std::path::{PathBuf, absolute};

pub fn bl33() -> PathBuf {
    product().join("bl33.bin")
}

pub fn rom() -> PathBuf {
    product().join("qemu_fw.rom")
}

pub fn top() -> PathBuf {
    absolute(PathBuf::from("../arm-trusted-firmware")).unwrap()
}

fn product() -> PathBuf {
    top().join("build/qemu/debug")
}
