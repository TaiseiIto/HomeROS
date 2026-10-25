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
    header: super::Header,
    alloc_descriptor: Header,
}

impl Hob {
    fn table(&self) -> &dyn Table {
        if self.alloc_descriptor.name == Stack::GUID.parse().unwrap() {
            unsafe { &*(self as *const Self as *const Stack) }
        } else {
            self
        }
    }
}

impl Debug for Hob {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        self.table().fmt(formatter)
    }
}

impl Table for Hob {}

impl super::Hob for Hob {}

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
    header: super::Header,
    alloc_descriptor: Header,
}

impl Stack {
    const GUID: &str = "4ed4bf27-4092-42e9-807d-527b1d00c9bd";
}

impl Table for Stack {}
