use crate::command::run;

mod boot;

pub fn all() {
    boot::all();
    xtask();
}

fn xtask() {
    run("cargo clippy --package xtask --all-features -- -D warnings")
}
