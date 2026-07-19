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
    let (mut args, arg, _, _): (Vec<String>, String, bool, bool) = command.chars().fold(
        (Vec::new(), String::new(), false, false),
        |(mut args, mut arg, in_quotation, in_double_quotation), character| match (
            character,
            in_quotation,
            in_double_quotation,
        ) {
            ('\'', false, false) => (args, arg, true, false),
            ('\'', true, false) => (args, arg, false, false),
            ('\'', _, true) => unreachable!(),
            ('"', false, false) => (args, arg, false, true),
            ('"', false, true) => (args, arg, false, false),
            ('"', true, _) => unreachable!(),
            (' ', false, false) | ('\t', false, false) => {
                args.push(arg);
                (args, String::new(), false, false)
            }
            (character, false, false) => {
                arg.push(character);
                (args, arg, false, false)
            }
            (character, true, false) => {
                arg.push(character);
                (args, arg, true, false)
            }
            (character, false, true) => {
                arg.push(character);
                (args, arg, false, true)
            }
            (_, true, true) => unreachable!(),
        },
    );
    args.push(arg);
    let mut args = args.into_iter();
    let mut command: std::process::Command = std::process::Command::new(args.next().unwrap());
    for arg in args {
        command.arg(arg);
    }
    command
}
