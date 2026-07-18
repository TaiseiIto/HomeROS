use crate::{docker, git, time};

pub fn attach() {
    build();
    let container: String = container();
    assert!(docker::container::exists(&container));
    assert!(docker::container::runs(&container));
    docker::container::attach(&container);
}

pub fn privilege(gpg_key: &std::path::Path, ssh_key: &std::path::Path) {
    build();
    let container: String = container();
    docker::container::copy(gpg_key, &container, &gpg_key_destination());
    docker::container::copy(ssh_key, &container, &ssh_key_destination());
}

pub fn remove() {
    let image: String = image();
    let container: String = container();
    if docker::container::runs(&container) {
        docker::container::stop(&container);
    }
    if docker::container::exists(&container) {
        docker::container::remove(&container);
    }
    if docker::image::exists(&image) {
        docker::image::remove(&image);
    }
}

fn build() {
    let image: String = image();
    let container: String = container();
    let dockerfile: std::path::PathBuf = dockerfile();
    assert!(dockerfile.exists());
    let arguments: std::collections::BTreeMap<String, String> = [
        ("CACHE_BUSTER", time::unix()),
        ("DOMAIN", git::domain()),
        ("DEVELOPER", git::developer()),
        ("PRODUCT", git::product()),
        ("BRANCH", git::branch()),
        ("TIMEZONE", time::zone()),
    ]
    .into_iter()
    .map(|(key, value)| (key.to_string(), value))
    .collect();
    if !docker::image::exists(&image) {
        docker::image::build(&image, &dockerfile, arguments);
    }
    if !docker::container::exists(&container) {
        docker::container::create(&image, &container);
    }
    if !docker::container::runs(&container) {
        docker::container::start(&container);
    }
}

fn container() -> String {
    git::product().to_lowercase()
}

fn dockerfile() -> std::path::PathBuf {
    std::path::PathBuf::from(".docker/Dockerfile")
}

fn gpg_key_destination() -> std::path::PathBuf {
    let mut path: std::path::PathBuf = home_directory();
    path.push(".gnupg");
    path
}

fn home_directory() -> std::path::PathBuf {
    build();
    docker::container::home_directory(&container())
}

fn image() -> String {
    git::product().to_lowercase()
}

fn ssh_key_destination() -> std::path::PathBuf {
    let mut path: std::path::PathBuf = home_directory();
    path.push(".ssh");
    path.push(&git::domain());
    path.push("key");
    path
}
