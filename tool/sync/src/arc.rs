use {
    alloc::boxed::Box,
    core::{
        cell::UnsafeCell,
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
    weak: Weak<T>,
}

impl<T> Arc<T> {
    pub fn downgrade(arc: &Self) -> Weak<T> {
        arc.weak.clone()
    }

    pub fn get_mut(arc: &mut Self) -> Option<&mut T> {
        (arc.weak.data().alloc_ref_count.load(Relaxed) == 1).then(|| {
            fence(Acquire);
            unsafe { arc.weak.ptr.as_mut() }
                .data
                .get_mut()
                .as_mut()
                .unwrap()
        })
    }

    pub fn new(data: T) -> Self {
        Self {
            weak: Weak {
                ptr: NonNull::from(Box::leak(Box::new(Data {
                    alloc_ref_count: AtomicUsize::new(1),
                    data_ref_count: AtomicUsize::new(1),
                    data: UnsafeCell::new(Some(data)),
                }))),
            },
        }
    }

    fn data(&self) -> &Data<T> {
        unsafe { self.weak.ptr.as_ref() }
    }
}

impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        let weak: Weak<T> = self.weak.clone();
        weak.data().data_ref_count.fetch_add(1, Relaxed);
        Self { weak }
    }
}

impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        let ptr: *mut Option<T> = self.weak.data().data.get();
        unsafe { &*ptr }.as_ref().unwrap()
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        if self.weak.data().data_ref_count.fetch_sub(1, Release) == 1 {
            fence(Acquire);
            let ptr: *mut Option<T> = self.weak.data().data.get();
            unsafe {
                *ptr = None;
            }
        }
    }
}

unsafe impl<T: Send + Sync> Send for Arc<T> {}
unsafe impl<T: Send + Sync> Sync for Arc<T> {}

pub struct Weak<T> {
    ptr: NonNull<Data<T>>,
}

impl<T> Weak<T> {
    pub fn upgrade(&self) -> Option<Arc<T>> {
        let mut n = self.data().data_ref_count.load(Relaxed);
        loop {
            if n == 0 {
                return None;
            }
            assert!(n < usize::MAX);
            if let Err(e) =
                self.data()
                    .data_ref_count
                    .compare_exchange_weak(n, n + 1, Relaxed, Relaxed)
            {
                n = e;
                continue;
            }
            return Some(Arc { weak: self.clone() });
        }
    }

    fn data(&self) -> &Data<T> {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T> Clone for Weak<T> {
    fn clone(&self) -> Self {
        self.data().alloc_ref_count.fetch_add(1, Relaxed);
        Self { ptr: self.ptr }
    }
}

impl<T> Drop for Weak<T> {
    fn drop(&mut self) {
        if self.data().alloc_ref_count.fetch_sub(1, Release) == 1 {
            fence(Acquire);
            unsafe {
                drop(Box::from_raw(self.ptr.as_ptr()));
            }
        }
    }
}

unsafe impl<T: Send + Sync> Send for Weak<T> {}
unsafe impl<T: Send + Sync> Sync for Weak<T> {}

struct Data<T> {
    alloc_ref_count: AtomicUsize,
    data_ref_count: AtomicUsize,
    data: UnsafeCell<Option<T>>,
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
