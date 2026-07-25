use {crate::git::product, std::path::PathBuf};

mod boot;

pub fn build() {
    boot::build(&destination());
}

pub fn destination() -> PathBuf {
    PathBuf::from(&format!("target/{}", product()))
}
