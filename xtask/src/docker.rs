pub fn build(image: &str, dockerfile: &std::path::Path) {
    if !image_exists(image) {
        std::process::Command::new("docker")
            .args([
                "build",
                "--tag",
                image,
                dockerfile.parent().unwrap().to_str().unwrap(),
            ])
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .status()
            .unwrap();
    }
}

pub fn create(image: &str, container: &str) {
    if !container_exists(container) {
        std::process::Command::new("docker")
            .args([
                "create",
                "--interactive",
                "--tty",
                "--name",
                container,
                image,
                "/bin/bash",
            ])
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .status()
            .unwrap();
    }
}

pub fn remove_container(container: &str) {
    if container_exists(container) {
        std::process::Command::new("docker")
            .args(["rm", container])
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .status()
            .unwrap();
    }
}

pub fn remove_image(image: &str) {
    if image_exists(image) {
        std::process::Command::new("docker")
            .args(["rmi", image])
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .status()
            .unwrap();
    }
}

pub fn stop_container(container: &str) {
    if container_runs(container) {
        std::process::Command::new("docker")
            .args(["stop", container])
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .status()
            .unwrap();
    }
}

fn container_exists(container: &str) -> bool {
    std::process::Command::new("docker")
        .args([
            "ps",
            "--all",
            "--format",
            "{{.Names}}",
            "--filter",
            &format!("name=^{}$", container),
        ])
        .output()
        .unwrap()
        .stdout
        .len()
        != 0
}

fn container_runs(container: &str) -> bool {
    std::process::Command::new("docker")
        .args([
            "ps",
            "--format",
            "{{.Names}}",
            "--filter",
            &format!("name=^{}$", container),
        ])
        .output()
        .unwrap()
        .stdout
        .len()
        != 0
}

fn image_exists(image: &str) -> bool {
    std::process::Command::new("docker")
        .args(["images", "--format", "{{.Repository}}", image])
        .output()
        .unwrap()
        .stdout
        .len()
        != 0
}
