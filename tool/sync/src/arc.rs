use {
    alloc::boxed::Box,
    core::{
        ops::Deref,
        ptr::NonNull,
        sync::atomic::{
            AtomicUsize,
            Ordering::{Acquire, Relaxed, Release},
            fence,
        },
    },
};

pub struct Arc<T> {
    ptr: NonNull<Data<T>>,
}

impl<T> Arc<T> {
    pub fn get_mut(arc: &mut Self) -> Option<&mut T> {
        (arc.data().ref_count.load(Relaxed) == 1).then(|| {
            fence(Acquire);
            unsafe { &mut arc.ptr.as_mut().data }
        })
    }

    pub fn new(data: T) -> Self {
        Self {
            ptr: NonNull::from(Box::leak(Box::new(Data {
                ref_count: AtomicUsize::new(1),
                data,
            }))),
        }
    }

    fn data(&self) -> &Data<T> {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        self.data().ref_count.fetch_add(1, Relaxed);
        Self { ptr: self.ptr }
    }
}

impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.data().data
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        if self.data().ref_count.fetch_sub(1, Release) == 1 {
            fence(Acquire);
            unsafe {
                drop(Box::from_raw(self.ptr.as_ptr()));
            }
        }
    }
}

unsafe impl<T: Send + Sync> Send for Arc<T> {}
unsafe impl<T: Send + Sync> Sync for Arc<T> {}

struct Data<T> {
    ref_count: AtomicUsize,
    data: T,
}

#[cfg(test)]
mod tests {
    use {super::*, std::thread};

    #[test]
    fn test() {
        static NUM_DROPS: AtomicUsize = AtomicUsize::new(0);

        struct DetectDrop;

        impl Drop for DetectDrop {
            fn drop(&mut self) {
                NUM_DROPS.fetch_add(1, Relaxed);
            }
        }

        let x = Arc::new(("hello", DetectDrop));
        let y = x.clone();
        let t = thread::spawn(move || {
            assert_eq!(x.0, "hello");
        });
        assert_eq!(y.0, "hello");
        t.join().unwrap();
        assert_eq!(NUM_DROPS.load(Relaxed), 0);
        drop(y);
        assert_eq!(NUM_DROPS.load(Relaxed), 1);
    }
}
