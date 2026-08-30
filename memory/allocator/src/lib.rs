#![no_std]

extern crate alloc;

use {
    alloc::alloc::Layout,
    core::{alloc::GlobalAlloc, cell::UnsafeCell},
    sync::spin::Lock,
};

#[global_allocator]
static GLOBAL: Global = Global::new();

struct Global(Lock<UnsafeCell<Allocator>>);

impl Global {
    const fn new() -> Self {
        Self(Lock::new(UnsafeCell::new(Allocator::new())))
    }
}

unsafe impl GlobalAlloc for Global {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unimplemented!();
    }

    unsafe fn dealloc(&self, address: *mut u8, layout: Layout) {
        unimplemented!();
    }
}

enum Allocator {
    Uninitialized,
}

impl Allocator {
    const fn new() -> Self {
        Self::Uninitialized
    }
}
