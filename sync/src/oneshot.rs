use core::{
    cell::{OnceCell, UnsafeCell},
    fmt::Debug,
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
pub struct Channel<T: Debug> {
    message: UnsafeCell<OnceCell<T>>,
    in_use: AtomicBool,
    ready: AtomicBool,
}

impl<T: Debug> Channel<T> {
    pub fn is_ready(&self) -> bool {
        self.ready.load(Relaxed)
    }

    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(OnceCell::new()),
            in_use: AtomicBool::new(false),
            ready: AtomicBool::new(false),
        }
    }

    pub fn receive(&self) -> T {
        if !self.ready.swap(false, Acquire) {
            panic!();
        }
        unsafe { &mut *self.message.get() }.take().unwrap()
    }

    pub fn send(&self, message: T) {
        if self.in_use.swap(true, Relaxed) {
            panic!();
        }
        unsafe { &mut *self.message.get() }.set(message).unwrap();
        self.ready.store(true, Release);
    }
}

impl<T: Debug> Drop for Channel<T> {
    fn drop(&mut self) {
        self.message.get_mut().take();
    }
}

unsafe impl<T: Debug> Sync for Channel<T> where T: Send {}

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
