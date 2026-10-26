use {
    crate::{
        Guid,
        service::boot::memory::{self, address::Physical},
    },
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [EFI_HOB_MEMORY_ALLOCATION](https://uefi.org/specs/PI/1.10/V3_HOB_Code_Definitions.html#efi-hob-memory-allocation)
#[repr(C)]
pub struct Hob {
    header: super::super::Header,
    alloc_descriptor: Header,
}

impl Hob {
    const STACK: &str = "4ed4bf27-4092-42e9-807d-527b1d00c9bd";
    const BSP_STORE: &str = "564b33cd-c92a-4593-90bf-2473e43c6322";
    const MODULE: &str = "f8e21975-0899-4f58-a4be-5525a9c6d77a";

    fn table(&self) -> &dyn Table {
        if self.alloc_descriptor.name == Self::STACK.parse().unwrap() {
            unsafe { &*(self as *const Self as *const Stack) }
        } else if self.alloc_descriptor.name == Self::BSP_STORE.parse().unwrap() {
            unsafe { &*(self as *const Self as *const BspStore) }
        } else if self.alloc_descriptor.name == Self::MODULE.parse().unwrap() {
            unsafe { &*(self as *const Self as *const Module) }
        } else {
            unsafe { &*(self as *const Self as *const Other) }
        }
    }
}

impl Debug for Hob {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        self.table().fmt(formatter)
    }
}

impl Table for Hob {}

impl super::super::Hob for Hob {}

/// # References
/// * [EFI_HOB_MEMORY_ALLOCATION](https://uefi.org/specs/PI/1.10/V3_HOB_Code_Definitions.html#efi-hob-memory-allocation)
#[derive(Debug)]
#[repr(C)]
struct Header {
    name: Guid,
    memory_base_address: Physical,
    memory_length: u64,
    memory_type: memory::Type,
    reserved: [u8; 4],
}

trait Table: Debug {}

/// # References
/// * [EFI_HOB_MEMORY_ALLOCATION_STACK](https://uefi.org/specs/PI/1.10/V3_HOB_Code_Definitions.html#efi-hob-memory-allocation-stack)
#[derive(Debug)]
#[repr(C)]
struct Stack {
    header: super::super::Header,
    alloc_descriptor: Header,
}

impl Table for Stack {}

/// # References
/// * [EFI_HOB_MEMORY_ALLOCATION_BSP_STORE](https://uefi.org/specs/PI/1.10/V3_HOB_Code_Definitions.html#efi-hob-memory-allocation-bsp-store)
#[derive(Debug)]
#[repr(C)]
struct BspStore {
    header: super::super::Header,
    alloc_descriptor: Header,
}

impl Table for BspStore {}

/// # References
/// * [EFI_HOB_MEMORY_ALLOCATION_MODULE](https://uefi.org/specs/PI/1.10/V3_HOB_Code_Definitions.html#efi-hob-memory-allocation-module)
#[derive(Debug)]
#[repr(C)]
struct Module {
    header: super::super::Header,
    alloc_descriptor: Header,
    module_name: Guid,
    entry_point: Physical,
}

impl Table for Module {}

#[derive(Debug)]
#[repr(C)]
struct Other {
    header: super::super::Header,
    alloc_descriptor: Header,
}

impl Table for Other {}
