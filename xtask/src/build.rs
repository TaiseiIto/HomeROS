use {crate::git::product, std::path::PathBuf};

mod boot;

pub fn all() {
    boot::all(&destination());
}

pub fn destination() -> PathBuf {
    PathBuf::from(&format!("target/{}", product()))
}
