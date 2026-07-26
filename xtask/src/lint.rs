use crate::{command::run, product};

pub fn lint() {
    product::lint();
    xtask();
}

fn xtask() {
    run("cargo clippy --package xtask --all-features -- -D warnings")
}
