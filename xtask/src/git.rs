use crate::command::{get_stdout, run};

pub fn add_rust_sources() {
    run("git add *.rs");
}

pub fn branch() -> String {
    get_stdout("git branch --show-current")
}

pub fn developer() -> String {
    extract("/:", 1)
}

pub fn domain() -> String {
    extract("/:@", 2)
}

pub fn email() -> String {
    get_stdout("git config user.email")
}

pub fn product() -> String {
    extract("./", 1)
}

fn extract(delimiters: &str, backward_index: usize) -> String {
    let delimiters: Vec<char> = delimiters.chars().collect();
    let delimiters: &[char] = &delimiters;
    let words: Vec<String> = url()
        .as_str()
        .split(delimiters)
        .map(|word| word.to_string())
        .collect();
    words.into_iter().rev().nth(backward_index).unwrap()
}

fn url() -> String {
    get_stdout("git remote get-url origin")
}
