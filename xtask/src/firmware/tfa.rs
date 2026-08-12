use std::path::{PathBuf, absolute};

pub fn bl33() -> PathBuf {
    product().join("bl33.bin")
}

pub fn fip() -> PathBuf {
    product().join("fip.bin")
}

pub fn top() -> PathBuf {
    absolute(PathBuf::from("../arm-trusted-firmware")).unwrap()
}

fn product() -> PathBuf {
    top().join("build/qemu/debug")
}
