use crate::{docker, git, time};

pub fn build() {
    let image: String = image();
    let container: String = container();
    let dockerfile: std::path::PathBuf = dockerfile();
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
    docker::container::attach(&container);
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

fn container() -> String {
    git::product().to_lowercase()
}

fn image() -> String {
    git::product().to_lowercase()
}

fn dockerfile() -> std::path::PathBuf {
    std::path::PathBuf::from(".docker/Dockerfile")
}
