pub fn get_stdout(command: &str) -> String {
    String::from_utf8(new(command).output().unwrap().stdout)
        .unwrap()
        .trim_end()
        .to_string()
}

pub fn run(command: &str) {
    let success: bool = new(command)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()
        .unwrap()
        .success();
    assert!(success);
}

fn new(command: &str) -> std::process::Command {
    let mut args: std::str::SplitAsciiWhitespace = command.split_ascii_whitespace();
    let mut command: std::process::Command = std::process::Command::new(args.next().unwrap());
    for arg in args {
        command.arg(arg);
    }
    command
}
