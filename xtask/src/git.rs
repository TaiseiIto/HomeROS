pub fn name() -> String {
    url()
        .as_str()
        .split('/')
        .last()
        .unwrap()
        .split('.')
        .next()
        .unwrap()
        .to_string()
}

fn url() -> String {
    String::from_utf8(
        std::process::Command::new("git")
            .args(["remote", "get-url", "origin"])
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap()
}
