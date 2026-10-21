use core::{
    hint::spin_loop,
    sync::atomic::{AtomicBool, Ordering::Relaxed},
};

static WAIT: AtomicBool = AtomicBool::new(true);

pub fn wait() {
    while WAIT.load(Relaxed) {
        breakpoint();
        spin_loop();
    }
}

fn breakpoint() {}
