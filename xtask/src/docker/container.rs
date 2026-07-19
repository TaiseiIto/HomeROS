use std::io::Write;
use {super::Image, crate::command};

pub fn attach(id_file: &std::path::Path) {
    assert!(exists(id_file));
    assert!(runs(id_file));
    command::run(&format!("docker attach {}", read_id(id_file)));
}

pub fn copy(source: &std::path::Path, id_file: &std::path::Path, destination: &std::path::Path) {
    assert!(exists(id_file));
    assert!(runs(id_file));
    if let Some(destination_directory) = destination.parent() {
        make_directory(id_file, destination_directory);
    }
    command::run(&format!(
        "docker cp {} {}:{}",
        source.to_str().unwrap(),
        read_id(id_file),
        destination.to_str().unwrap()
    ));
}

pub fn create(image: &Image, id_file: &std::path::Path) {
    assert!(!exists(id_file));
    let id_file_id: String = command::get_stdout(&format!(
        "docker create --interactive --tty {} /bin/bash",
        image.read_id()
    ));
    std::fs::write(id_file, id_file_id).unwrap();
}

pub fn execute(id_file: &std::path::Path, command: &str) -> String {
    assert!(exists(id_file));
    assert!(runs(id_file));
    command::get_stdout(&format!("docker exec {} {}", read_id(id_file), command))
}

pub fn exists(id_file: &std::path::Path) -> bool {
    id_file.exists() && id_file.is_file() && {
        let my_id: String = read_id(id_file);
        command::test(&format!("docker inspect {}", my_id))
    }
}

pub fn groups(id_file: &std::path::Path) -> Vec<String> {
    assert!(exists(id_file));
    assert!(runs(id_file));
    let mut groups: Vec<String> = execute(id_file, "groups")
        .split_whitespace()
        .map(|group| group.to_string())
        .collect();
    groups.sort();
    groups
}

pub fn home_directory(id_file: &std::path::Path) -> std::path::PathBuf {
    assert!(exists(id_file));
    assert!(runs(id_file));
    execute(id_file, "printenv HOME").into()
}

pub fn remove(id_file: &std::path::Path) {
    assert!(exists(id_file));
    assert!(!runs(id_file));
    command::run(&format!("docker rm {}", read_id(id_file)));
}

pub fn runs(id_file: &std::path::Path) -> bool {
    exists(id_file)
        && command::get_stdout(&format!(
            "docker inspect -f {{{{.State.Running}}}} {}",
            read_id(id_file)
        )) == "true"
}

pub fn start(id_file: &std::path::Path) {
    assert!(exists(id_file));
    assert!(!runs(id_file));
    command::run(&format!("docker start {}", read_id(id_file)));
}

pub fn stop(id_file: &std::path::Path) {
    assert!(exists(id_file));
    assert!(runs(id_file));
    command::run(&format!("docker stop {}", read_id(id_file)));
}

pub fn user(id_file: &std::path::Path) -> String {
    assert!(exists(id_file));
    assert!(runs(id_file));
    execute(id_file, "whoami")
}

pub fn write(id_file: &std::path::Path, destination: &std::path::Path, data: &str) {
    let mut temporary: tempfile::NamedTempFile = tempfile::NamedTempFile::new().unwrap();
    write!(temporary, "{}", data).unwrap();
    copy(temporary.path(), id_file, destination);
}

fn make_directory(id_file: &std::path::Path, directory: &std::path::Path) {
    execute(id_file, &format!("mkdir {}", directory.to_str().unwrap()));
}

fn read_id(id_file: &std::path::Path) -> String {
    assert!(id_file.exists());
    assert!(id_file.is_file());
    std::fs::read_to_string(id_file).unwrap()
}
