use {
    crate::Arc,
    core::{
        cell::{OnceCell, UnsafeCell},
        fmt::Debug,
        marker::{Send, Sync},
        ops::Drop,
        sync::atomic::{
            AtomicBool,
            Ordering::{Acquire, Relaxed, Release},
        },
    },
};

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let channel = Arc::new(Channel {
        message: UnsafeCell::new(OnceCell::new()),
        ready: AtomicBool::new(false),
    });
    (Sender(channel.clone()), Receiver(channel))
}

pub struct Sender<T>(Arc<Channel<T>>);

impl<T: Debug> Sender<T> {
    pub fn send(self, message: T) {
        unsafe { &mut *self.0.message.get() }.set(message).unwrap();
        self.0.ready.store(true, Release);
    }
}

pub struct Receiver<T>(Arc<Channel<T>>);

impl<T> Receiver<T> {
    pub fn is_ready(&self) -> bool {
        self.0.ready.load(Relaxed)
    }

    pub fn receive(self) -> T {
        assert!(self.0.ready.swap(false, Acquire));
        unsafe { &mut *self.0.message.get() }.take().unwrap()
    }
}

#[derive(Default)]
struct Channel<T> {
    message: UnsafeCell<OnceCell<T>>,
    ready: AtomicBool,
}

impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        unsafe { &mut *self.message.get() }.take();
    }
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

#[cfg(test)]
mod tests {
    use {
        super::*,
        std::thread::{self, Thread},
    };

    #[test]
    fn test() {
        thread::scope(|scope| {
            let (sender, receiver): (Sender<&str>, Receiver<&str>) = channel();
            let main_thread: Thread = thread::current();
            let message: &str = "Hello, World!";
            scope.spawn(move || {
                sender.send(message);
                main_thread.unpark();
            });
            while !receiver.is_ready() {
                thread::park();
            }
            assert_eq!(receiver.receive(), message);
        });
    }
}
