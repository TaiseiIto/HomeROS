use crate::command;

pub fn build(
    image: &str,
    dockerfile: &std::path::Path,
    arguments: std::collections::BTreeMap<String, String>,
) {
    assert!(!exists(image));
    let arguments: Vec<String> = arguments
        .iter()
        .map(|(key, value)| format!("--build-arg {}={}", key, value))
        .collect();
    let arguments: String = arguments.join(" ");
    command::run(&format!(
        "docker build --tag {} {} {}",
        image,
        dockerfile.parent().unwrap().to_str().unwrap(),
        arguments
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
