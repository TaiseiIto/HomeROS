#![no_std]

extern crate alloc;

#[cfg(use_temporary_memory_allocator)]
mod linked;

use {
    alloc::alloc::Layout,
    core::{alloc::GlobalAlloc, cell::UnsafeCell},
    sync::spin::Lock,
};

#[cfg(firmware = "uefi")]
use uefi::service::boot::memory::Map;

#[cfg(has_device_tree)]
use memory::Regions;

pub fn stabilize(
    #[cfg(has_device_tree)] regions: Regions<usize>,
    #[cfg(start_with_assembly)] boot_loader_head: usize,
    #[cfg(firmware = "uefi")] map: Map,
) {
    GLOBAL.stabilize(
        #[cfg(has_device_tree)]
        regions,
        #[cfg(start_with_assembly)]
        boot_loader_head,
        #[cfg(firmware = "uefi")]
        map,
    );
}

pub fn temporize(#[cfg(use_temporary_memory_allocator)] head: usize) {
    GLOBAL.temporize(
        #[cfg(use_temporary_memory_allocator)]
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

    fn stabilize(
        &self,
        #[cfg(has_device_tree)] regions: Regions<usize>,
        #[cfg(start_with_assembly)] boot_loader_head: usize,
        #[cfg(firmware = "uefi")] map: Map,
    ) {
        unsafe { &mut *self.0.lock().get() }.stabilize(
            #[cfg(has_device_tree)]
            regions,
            #[cfg(start_with_assembly)]
            boot_loader_head,
            #[cfg(firmware = "uefi")]
            map,
        );
    }

    fn temporize(&self, #[cfg(use_temporary_memory_allocator)] head: usize) {
        unsafe { &mut *self.0.lock().get() }.temporize(
            #[cfg(use_temporary_memory_allocator)]
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
    Stable(
        #[cfg(has_device_tree)] Regions<usize>,
        #[cfg(firmware = "uefi")] Map,
    ),
    Temporary(#[cfg(use_temporary_memory_allocator)] linked::List),
    Uninitialized,
}

impl Allocator {
    const fn new() -> Self {
        Self::Uninitialized
    }

    fn stabilize(
        &mut self,
        #[cfg(has_device_tree)] regions: Regions<usize>,
        #[cfg(start_with_assembly)] boot_loader_head: usize,
        #[cfg(firmware = "uefi")] map: Map,
    ) {
        *self = Self::Stable(
            #[cfg(has_device_tree)]
            regions,
            #[cfg(firmware = "uefi")]
            map,
        );
    }

    fn temporize(&mut self, #[cfg(use_temporary_memory_allocator)] head: usize) {
        *self = Self::Temporary(
            #[cfg(use_temporary_memory_allocator)]
            linked::List::new(head),
        );
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        match self {
            #[cfg(has_device_tree)]
            Self::Stable(regions) => unimplemented!(),
            #[cfg(firmware = "uefi")]
            Self::Stable(map) => unimplemented!(),
            #[cfg(use_temporary_memory_allocator)]
            Self::Temporary(linked_list) => unsafe { linked_list.alloc(layout) },
            #[cfg(firmware = "uefi")]
            Self::Temporary() => panic!(),
            Self::Uninitialized => panic!(),
        }
    }

    unsafe fn dealloc(&self, address: *mut u8, layout: Layout) {
        match self {
            #[cfg(has_device_tree)]
            Self::Stable(regions) => unimplemented!(),
            #[cfg(firmware = "uefi")]
            Self::Stable(map) => unimplemented!(),
            #[cfg(use_temporary_memory_allocator)]
            Self::Temporary(linked_list) => unsafe {
                linked_list.dealloc(address, layout);
            },
            #[cfg(firmware = "uefi")]
            Self::Temporary() => panic!(),
            Self::Uninitialized => panic!(),
        }
    }
}
