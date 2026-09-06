use crate::command::run;

pub fn test() {
    for package in ["memory", "sync"].as_slice().iter() {
        run(&format!("cargo test --package {}", package));
    }
}
