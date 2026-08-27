use core::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    sync::atomic::{
        AtomicBool,
        Ordering::{Acquire, Release},
    },
};

pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
}

impl<T> Channel<T> {
    pub fn is_read(&self) -> bool {
        self.ready.load(Acquire)
    }

    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false),
        }
    }

    pub unsafe fn receive(&self) -> T {
        unsafe { (*self.message.get()).assume_init_read() }
    }

    pub unsafe fn send(&self, message: T) {
        unsafe { &mut *self.message.get() }.write(message);
        self.ready.store(true, Release);
    }
}

unsafe impl<T> Sync for Channel<T> where T: Send {}
