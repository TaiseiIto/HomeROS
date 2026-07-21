use std::{
    collections::BTreeMap,
    io::Write,
    process::{Child, Command, Stdio},
};

pub fn get_stdout(command: &str) -> String {
    String::from_utf8(new(command).output().unwrap().stdout)
        .unwrap()
        .trim_end()
        .to_string()
}

pub fn give_stdin(command: &str, stdin: &[u8]) {
    let mut process: Child = new(command).stdin(Stdio::piped()).spawn().unwrap();
    process.stdin.take().unwrap().write_all(stdin).unwrap();
    process.wait().unwrap();
}

pub fn run(command: &str) {
    let success: bool = new(command)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .unwrap()
        .success();
    assert!(success);
}

pub fn test(command: &str) -> bool {
    new(command)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .unwrap()
        .success()
}

fn new(command: &str) -> Command {
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
    let (envs, command, args): (BTreeMap<String, String>, Option<Command>, Vec<String>) =
        args.into_iter().fold(
            (BTreeMap::new(), None, Vec::new()),
            |(mut envs, command, mut args), arg| match command {
                None => match arg.split_once('=') {
                    None => (envs, Some(Command::new(arg)), args),
                    Some((key, value)) => {
                        envs.insert(key.to_string(), value.to_string());
                        (envs, command, args)
                    }
                },
                Some(command) => {
                    args.push(arg);
                    (envs, Some(command), args)
                }
            },
        );
    let mut command: Command = command.unwrap();
    command.envs(envs);
    command.args(args);
    command
}
