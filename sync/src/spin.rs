use {
    arch::pause,
    core::sync::atomic::{AtomicBool, Ordering},
};

pub struct Lock {
    locked: AtomicBool,
}

impl Lock {
    pub const fn new() -> Self {
        Self {
            locked: AtomicBool::new(false),
        }
    }

    pub fn lock(&self) {
        while self.locked.swap(true, Ordering::Acquire) {
            pause();
        }
    }

    pub fn unlock(&self) {
        self.locked.store(false, Ordering::Release);
    }
}
