pub fn create(image: &str, container: &str) {
    if !exists(container) {
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

pub fn remove(container: &str) {
    if exists(container) {
        std::process::Command::new("docker")
            .args(["rm", container])
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .status()
            .unwrap();
    }
}

pub fn stop(container: &str) {
    if runs(container) {
        std::process::Command::new("docker")
            .args(["stop", container])
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .status()
            .unwrap();
    }
}

fn exists(container: &str) -> bool {
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

fn runs(container: &str) -> bool {
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
