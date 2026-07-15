use crate::command;

pub fn create(image: &str, container: &str) {
    if !exists(container) {
        command::run(&format!(
            "docker create --interactive --tty --name {} {} /bin/bash",
            container, image
        ));
    }
}

pub fn remove(container: &str) {
    if exists(container) {
        command::run(&format!("docker rm {}", container));
    }
}

pub fn stop(container: &str) {
    if runs(container) {
        command::run(&format!("docker stop {}", container));
    }
}

fn exists(container: &str) -> bool {
    !command::get_stdout(&format!(
        "docker ps --all --format {{{{.Names}}}} --filter name=^{}$",
        container
    ))
    .is_empty()
}

fn runs(container: &str) -> bool {
    !command::get_stdout(&format!(
        "docker ps --format {{{{.Names}}}} --filter name=^{}$",
        container
    ))
    .is_empty()
}
