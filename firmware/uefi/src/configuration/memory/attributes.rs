use crate::{Guid, guid};

/// # References
/// * [EFI_MEMORY_ATTRIBUTES_TABLE](https://uefi.org/specs/UEFI/2.11/04_EFI_System_Table.html#efi-memory-attributes-table)
#[derive(Debug)]
#[repr(C)]
pub struct Table {
    version: u32,
    number_of_entries: u32,
    descriptor_size: u32,
    flags: u32,
}

impl Table {
    pub const GUID: Guid = guid!("dcfa911d-26eb-469f-a220-38b7dc461220");
}

impl super::super::Table for Table {}
