use {crate::git::product, std::path::PathBuf};

mod boot;

pub fn all() {
    let destination: PathBuf = PathBuf::from(&format!("target/{}", product()));
    boot::all(&destination);
}
