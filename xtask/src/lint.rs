use crate::command::run;

mod boot;

pub fn lint() {
    boot::lint();
    xtask();
}

fn xtask() {
    run("cargo clippy --package xtask --all-features -- -D warnings")
}
