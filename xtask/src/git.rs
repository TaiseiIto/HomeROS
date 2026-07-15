use crate::command;

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
    command::get_stdout("git remote get-url origin")
}
