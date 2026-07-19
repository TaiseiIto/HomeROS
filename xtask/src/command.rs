use std::io::Write;

pub fn get_stdout(command: &str) -> String {
    String::from_utf8(new(command).output().unwrap().stdout)
        .unwrap()
        .trim_end()
        .to_string()
}

pub fn give_stdin(command: &str, stdin: &[u8]) {
    let mut process: std::process::Child = new(command)
        .stdin(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    process.stdin.take().unwrap().write_all(stdin).unwrap();
    process.wait().unwrap();
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

pub fn test(command: &str) -> bool {
    new(command)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .unwrap()
        .success()
}

fn new(command: &str) -> std::process::Command {
    let mut args: std::str::SplitAsciiWhitespace = command.split_ascii_whitespace();
    let mut command: std::process::Command = std::process::Command::new(args.next().unwrap());
    for arg in args {
        command.arg(arg);
    }
    command
}
