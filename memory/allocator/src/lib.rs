#![no_std]

extern crate alloc;

mod buddy;
#[cfg(use_temporary_memory_allocator)]
mod linked;

use {
    alloc::alloc::Layout,
    core::{
        alloc::GlobalAlloc,
        cell::UnsafeCell,
        fmt::{Debug, Formatter, Result},
        ops::Range,
    },
    sync::spin::Lock,
};

#[cfg(firmware = "uefi")]
use uefi::service::boot::memory::Map;

#[cfg(has_device_tree)]
use memory::{Region, Regions};

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
pub static GLOBAL: Global = Global::new();

pub struct Global(Lock<UnsafeCell<Allocator>>);

impl Global {
    pub fn get(&self) -> &Lock<UnsafeCell<Allocator>> {
        &self.0
    }

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

pub enum Allocator {
    Stable {
        #[cfg(has_device_tree)]
        regions: Regions<usize>,
        #[cfg(all(start_with_assembly, use_temporary_memory_allocator))]
        boot_loader: Region<usize>,
        #[cfg(firmware = "uefi")]
        map: Map,
    },
    Temporary(#[cfg(use_temporary_memory_allocator)] linked::List),
    Uninitialized,
}

impl Allocator {
    const fn new() -> Self {
        Self::Uninitialized
    }

    fn buddy_ranges(&self) -> impl Iterator<Item = Range<usize>> {
        match self {
            #[cfg(use_temporary_memory_allocator)]
            Self::Stable {
                regions,
                boot_loader,
            } => regions
                .iter()
                .flat_map(|region| region.subtract(boot_loader))
                .map(|region| region.range()),
            #[cfg(firmware = "uefi")]
            Self::Stable { map } => map
                .iter()
                .filter(|descriptor| descriptor.is_allocatable())
                .map(|descriptor| descriptor.range()),
            _ => panic!(),
        }
    }

    fn buddy_roots(&self) -> impl Iterator<Item = Range<usize>> {
        self.buddy_ranges().flat_map(Into::<buddy::Roots>::into)
    }

    fn stabilize(
        &mut self,
        #[cfg(has_device_tree)] regions: Regions<usize>,
        #[cfg(start_with_assembly)] boot_loader_head: usize,
        #[cfg(firmware = "uefi")] map: Map,
    ) {
        #[cfg(use_temporary_memory_allocator)]
        let linked_list: &linked::List = if let Self::Temporary(linked_list) = self {
            linked_list
        } else {
            panic!();
        };
        *self = Self::Stable {
            #[cfg(has_device_tree)]
            regions,
            #[cfg(all(start_with_assembly, use_temporary_memory_allocator))]
            boot_loader: (boot_loader_head..linked_list.tail()).try_into().unwrap(),
            #[cfg(firmware = "uefi")]
            map,
        };
        for region in self.buddy_roots() {
            buddy::NodeList::initialize(&region);
        }
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
            Self::Stable {
                regions,
                boot_loader,
            } => unimplemented!(),
            #[cfg(firmware = "uefi")]
            Self::Stable { map } => unimplemented!(),
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
            Self::Stable {
                regions,
                boot_loader,
            } => unimplemented!(),
            #[cfg(firmware = "uefi")]
            Self::Stable { map } => unimplemented!(),
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

impl Debug for Allocator {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            #[cfg(use_temporary_memory_allocator)]
            Self::Stable {
                regions,
                boot_loader,
            } => formatter.debug_list().entries(self.buddy_roots()).finish(),
            #[cfg(firmware = "uefi")]
            Self::Stable { map } => formatter.debug_list().entries(self.buddy_roots()).finish(),
            #[cfg(use_temporary_memory_allocator)]
            Self::Temporary(linked_list) => formatter
                .debug_tuple("Temporary")
                .field(linked_list)
                .finish(),
            #[cfg(firmware = "uefi")]
            Self::Temporary() => formatter.write_str("Temporary"),
            Self::Uninitialized => formatter.write_str("Uninitialized"),
        }
    }
}
