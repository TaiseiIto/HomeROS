use crate::command::run;

pub fn test() {
    for package in ["sync"].as_slice().iter() {
        run(&format!("cargo test --package {}", package));
    }
}
