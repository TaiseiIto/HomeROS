use crate::command;

pub fn attach(container: &str) {
    assert!(exists(container));
    assert!(runs(container));
    command::run(&format!("docker attach {}", container));
}

pub fn copy(source: &std::path::Path, container: &str, destination: &std::path::Path) {
    assert!(exists(container));
    assert!(runs(container));
    if let Some(destination_directory) = destination.parent() {
        make_directory(container, destination_directory);
    }
    command::run(&format!(
        "docker cp {} {}:{}",
        source.to_str().unwrap(),
        container,
        destination.to_str().unwrap()
    ));
}

pub fn create(image: &str, container: &str) {
    assert!(!exists(container));
    command::run(&format!(
        "docker create --interactive --tty --name {} {} /bin/bash",
        container, image
    ));
}

pub fn execute(container: &str, command: &str) -> String {
    assert!(exists(container));
    assert!(runs(container));
    command::get_stdout(&format!("docker exec {} {}", container, command))
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
    execute(container, "printenv HOME").into()
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

pub fn write(container: &str, destination: &std::path::Path, data: &str) {
    let temporary: tempfile::NamedTempFile = tempfile::NamedTempFile::new().unwrap();
    write!(temporary, data);
    copy(temporary.path(), container, destination);
}

fn make_directory(container: &str, directory: &std::path::Path) {
    execute(container, &format!("mkdir {}", directory.to_str().unwrap()));
}
