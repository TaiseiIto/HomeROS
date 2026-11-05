use {
    crate::{Guid, guid},
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [EFI_MEMORY_ATTRIBUTES_TABLE](https://uefi.org/specs/UEFI/2.11/04_EFI_System_Table.html#efi-memory-attributes-table)
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

impl Debug for Table {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Table")
            .field("version", &self.version)
            .field("number_of_entries", &self.number_of_entries)
            .field("descriptor_size", &self.descriptor_size)
            .field("flags", &self.flags)
            .finish()
    }
}

impl super::super::Table for Table {}
