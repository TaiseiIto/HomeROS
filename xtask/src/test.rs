use crate::command::run;

pub fn test() {
    for package in ["memory", "regex", "sync"].as_slice().iter() {
        run(&format!("cargo test --package {}", package));
    }
}
