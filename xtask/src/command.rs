pub fn get_stdout(command: &str) -> String {
    let mut args: std::str::SplitAsciiWhitespace = command.split_ascii_whitespace();
    let mut command: std::process::Command = std::process::Command::new(args.next().unwrap());
    for arg in args {
        command.arg(arg);
    }
    String::from_utf8(command.output().unwrap().stdout).unwrap()
}
