pub fn build(image: &str, dockerfile: &std::path::Path) {
    if !exists(image) {
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

pub fn remove(image: &str) {
    if exists(image) {
        std::process::Command::new("docker")
            .args(["rmi", image])
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .status()
            .unwrap();
    }
}

fn exists(image: &str) -> bool {
    std::process::Command::new("docker")
        .args(["images", "--format", "{{.Repository}}", image])
        .output()
        .unwrap()
        .stdout
        .len()
        != 0
}
