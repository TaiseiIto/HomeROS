use crate::command;

pub fn build(image: &str, dockerfile: &std::path::Path) {
    assert!(!exists(image));
    command::run(&format!(
        "docker build --tag {} {}",
        image,
        dockerfile.parent().unwrap().to_str().unwrap()
    ));
}

pub fn exists(image: &str) -> bool {
    !command::get_stdout(&format!(
        "docker images --format {{{{.Repository}}}} {}",
        image
    ))
    .is_empty()
}

pub fn remove(image: &str) {
    assert!(exists(image));
    command::run(&format!("docker rmi {}", image));
}
