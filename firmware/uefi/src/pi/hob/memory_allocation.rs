use crate::{
    Guid,
    service::boot::memory::{self, address::Physical},
};

/// # References
/// * [EFI_HOB_MEMORY_ALLOCATION](https://uefi.org/specs/PI/1.10/V3_HOB_Code_Definitions.html#efi-hob-memory-allocation)
#[derive(Debug)]
#[repr(C)]
pub struct Hob {
    header: super::Header,
    alloc_descriptor: Header,
}

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
