use {
    arch::pause,
    core::{
        cell::UnsafeCell,
        marker::{Send, Sync},
        ops::{Deref, DerefMut, Drop},
        sync::atomic::{
            AtomicBool,
            Ordering::{Acquire, Release},
        },
    },
};

pub struct Guard<'a, T>(&'a Lock<T>);

impl<T> Deref for Guard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0.value.get() }
    }
}

impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.0.value.get() }
    }
}

impl<T> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        self.0.unlock();
    }
}

unsafe impl<T> Send for Guard<'_, T> where T: Send {}
unsafe impl<T> Sync for Guard<'_, T> where T: Sync {}

pub struct Lock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

impl<T> Lock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock<'a>(&'a self) -> Guard<'a, T> {
        while self.locked.swap(true, Acquire) {
            pause();
        }
        Guard(self)
    }

    pub fn unlock(&self) {
        self.locked.store(false, Release);
    }
}

unsafe impl<T> Sync for Lock<T> where T: Send {}
