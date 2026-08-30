#![no_std]

extern crate alloc;

#[cfg(any(firmware = "sbi", firmware = "tfa"))]
mod linked;

use {
    alloc::alloc::Layout,
    core::{alloc::GlobalAlloc, cell::UnsafeCell},
    sync::spin::Lock,
};

pub fn temporize(#[cfg(any(firmware = "sbi", firmware = "tfa"))] head: usize) {
    GLOBAL.temporize(
        #[cfg(any(firmware = "sbi", firmware = "tfa"))]
        head,
    );
}

#[global_allocator]
static GLOBAL: Global = Global::new();

struct Global(Lock<UnsafeCell<Allocator>>);

impl Global {
    const fn new() -> Self {
        Self(Lock::new(UnsafeCell::new(Allocator::new())))
    }

    fn temporize(&self, #[cfg(any(firmware = "sbi", firmware = "tfa"))] head: usize) {
        unsafe { &mut *self.0.lock().get() }.temporize(
            #[cfg(any(firmware = "sbi", firmware = "tfa"))]
            head,
        );
    }
}

unsafe impl GlobalAlloc for Global {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { (&*self.0.lock().get()).alloc(layout) }
    }

    unsafe fn dealloc(&self, address: *mut u8, layout: Layout) {
        unsafe {
            (&*self.0.lock().get()).dealloc(address, layout);
        }
    }
}

unsafe impl Send for Global {}
unsafe impl Sync for Global {}

enum Allocator {
    Temporary(#[cfg(any(firmware = "sbi", firmware = "tfa"))] linked::List),
    Uninitialized,
}

impl Allocator {
    const fn new() -> Self {
        Self::Uninitialized
    }

    fn temporize(&mut self, #[cfg(any(firmware = "sbi", firmware = "tfa"))] head: usize) {
        *self = Self::Temporary(
            #[cfg(any(firmware = "sbi", firmware = "tfa"))]
            linked::List::new(head),
        );
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        match self {
            #[cfg(any(firmware = "sbi", firmware = "tfa"))]
            Self::Temporary(linked_list) => unsafe { linked_list.alloc(layout) },
            #[cfg(firmware = "uefi")]
            Self::Temporary() => panic!(),
            Self::Uninitialized => panic!(),
        }
    }

    unsafe fn dealloc(&self, address: *mut u8, layout: Layout) {
        match self {
            #[cfg(any(firmware = "sbi", firmware = "tfa"))]
            Self::Temporary(linked_list) => unsafe {
                linked_list.dealloc(address, layout);
            },
            #[cfg(firmware = "uefi")]
            Self::Temporary() => panic!(),
            Self::Uninitialized => panic!(),
        }
    }
}
