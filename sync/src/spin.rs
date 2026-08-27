use {
    arch::pause,
    core::{
        cell::UnsafeCell,
        sync::atomic::{AtomicBool, Ordering},
    },
};

pub struct Lock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

impl Lock {
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock(&self) -> &mut T {
        while self.locked.swap(true, Ordering::Acquire) {
            pause();
        }
        unsafe { &mut *self.value.get() }
    }

    pub fn unlock(&self) {
        self.locked.store(false, Ordering::Release);
    }
}

unsafe impl<T> Sync for SpinLock<T> where T: Send {}
