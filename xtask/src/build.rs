mod boot;

pub fn all() {
    boot();
}

fn boot() {
    boot::aarch64();
    boot::x86_64();
}
