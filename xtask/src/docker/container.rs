use crate::command;

pub fn attach(container: &str) {
    assert!(exists(container));
    assert!(runs(container));
    command::run(&format!("docker attach {}", container));
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
