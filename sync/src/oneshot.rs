use core::{
    cell::UnsafeCell,
    marker::{Send, Sync},
    mem::MaybeUninit,
    ops::Drop,
    sync::atomic::{
        AtomicBool,
        Ordering::{Acquire, Relaxed, Release},
    },
};

/// # TODO
/// * Read 5.4 of [Rust Atomics and Locks](https://www.oreilly.co.jp/books/9784814400515/) after implementing Arc.
pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    in_use: AtomicBool,
    ready: AtomicBool,
}

impl<T> Channel<T> {
    pub fn is_ready(&self) -> bool {
        self.ready.load(Relaxed)
    }

    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            in_use: AtomicBool::new(false),
            ready: AtomicBool::new(false),
        }
    }

    pub fn receive(&self) -> T {
        if !self.ready.swap(false, Acquire) {
            panic!();
        }
        unsafe { (*self.message.get()).assume_init_read() }
    }

    pub fn send(&self, message: T) {
        if self.in_use.swap(true, Relaxed) {
            panic!();
        }
        unsafe { &mut *self.message.get() }.write(message);
        self.ready.store(true, Release);
    }
}

impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.ready.get_mut() {
            unsafe {
                self.message.get_mut().assume_init_drop();
            }
        }
    }
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

#[cfg(test)]
mod tests {
    use {super::*, std::thread};

    #[test]
    fn test() {
        let channel = Channel::new();
        let main_thread = thread::current();
        let message: &str = "Hello, World!";
        thread::scope(|thread_scope| {
            thread_scope.spawn(|| {
                channel.send(message);
                main_thread.unpark();
            });
            while !channel.is_ready() {
                thread::park();
            }
            assert_eq!(channel.receive(), message);
        });
    }
}
