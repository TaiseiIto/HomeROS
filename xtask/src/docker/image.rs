use crate::command;

pub fn build(
    id_file: &std::path::Path,
    dockerfile: &std::path::Path,
    arguments: std::collections::BTreeMap<String, String>,
) {
    assert!(!exists(id_file));
    let arguments: Vec<String> = arguments
        .iter()
        .map(|(key, value)| format!("--build-arg {}={}", key, value))
        .collect();
    let arguments: String = arguments.join(" ");
    command::run(&format!(
        "docker build --iidfile {} {} {}",
        id_file.to_str().unwrap(),
        dockerfile.parent().unwrap().to_str().unwrap(),
        arguments
    ));
}

pub fn exists(id_file: &std::path::Path) -> bool {
    id_file.exists() && id_file.is_file() && {
        let my_id: String = read_id(id_file);
        command::get_stdout("docker images --no-trunc --format {{{{.ID}}}}")
            .lines()
            .any(|id| id == my_id)
    }
}

pub fn read_id(id_file: &std::path::Path) -> String {
    assert!(id_file.exists());
    assert!(id_file.is_file());
    std::fs::read_to_string(id_file).unwrap()
}

pub fn remove(id_file: &std::path::Path) {
    assert!(exists(id_file));
    command::run(&format!("docker image rm {}", read_id(id_file)));
}

