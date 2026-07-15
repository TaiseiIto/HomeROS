use crate::command;

pub fn build(image: &str, dockerfile: &std::path::Path) {
    if !exists(image) {
        command::run(&format!(
            "docker build --tag {} {}",
            image,
            dockerfile.parent().unwrap().to_str().unwrap()
        ));
    }
}

pub fn remove(image: &str) {
    if exists(image) {
        command::run(&format!("docker rmi {}", image));
    }
}

fn exists(image: &str) -> bool {
    !command::get_stdout(&format!(
        "docker images --format {{{{.Repository}}}} {}",
        image
    ))
    .is_empty()
}
