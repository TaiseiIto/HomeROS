use crate::command;

pub fn attach(container: &str) {
    assert!(exists(container));
    assert!(runs(container));
    command::run(&format!("docker attach {}", container));
}

pub fn copy(source: &std::path::Path, container: &str, destination: &std::path::Path) {
    assert!(exists(container));
    assert!(runs(container));
    destination.parent().map(|destination_directory| make_directory(container, destination_directory));
    command::run(&format!("docker cp {} {}:{}", source.to_str().unwrap(), container, destination.to_str().unwrap()));
}

pub fn create(image: &str, container: &str) {
    assert!(!exists(container));
    command::run(&format!(
        "docker create --interactive --tty --name {} {} /bin/bash",
        container, image
    ));
}

pub fn exists(container: &str) -> bool {
    !command::get_stdout(&format!(
        "docker ps --all --format {{{{.Names}}}} --filter name=^{}$",
        container
    ))
    .is_empty()
}

pub fn home_directory(container: &str) -> std::path::PathBuf {
    assert!(exists(container));
    assert!(runs(container));
    get_stdout(container, "printenv HOME").into()
}

pub fn remove(container: &str) {
    assert!(exists(container));
    assert!(!runs(container));
    command::run(&format!("docker rm {}", container));
}

pub fn runs(container: &str) -> bool {
    !command::get_stdout(&format!(
        "docker ps --format {{{{.Names}}}} --filter name=^{}$",
        container
    ))
    .is_empty()
}

pub fn start(container: &str) {
    assert!(exists(container));
    assert!(!runs(container));
    command::run(&format!("docker start {}", container));
}

pub fn stop(container: &str) {
    assert!(exists(container));
    assert!(runs(container));
    command::run(&format!("docker stop {}", container));
}

fn make_directory(container: &str, directory: &std::path::Path) {
    get_stdout(container, &format!("mkdir {}", directory.to_str().unwrap()));
}

fn get_stdout(container: &str, command: &str) -> String {
    assert!(exists(container));
    assert!(runs(container));
    command::get_stdout(&format!(
        "docker exec {} {}",
        container,
        command
    ))
}

