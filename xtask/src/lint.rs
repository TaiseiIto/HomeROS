use crate::command;

mod boot;

pub fn all() {
    boot::all();
    xtask();
}

fn xtask() {
    command::run("cargo clippy --package xtask --all-features -- -D warnings")
}
