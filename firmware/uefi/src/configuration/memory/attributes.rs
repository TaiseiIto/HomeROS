use {
    crate::{Guid, guid, service::boot::memory::map::Descriptor},
    alloc::vec::Vec,
    core::{
        fmt::{Debug, Formatter, Result},
        marker::PhantomData,
    },
};

/// # References
/// * [EFI_MEMORY_ATTRIBUTES_TABLE](https://uefi.org/specs/UEFI/2.11/04_EFI_System_Table.html#efi-memory-attributes-table)
#[repr(C)]
pub struct Table {
    version: u32,
    number_of_entries: u32,
    descriptor_size: u32,
    flags: u32,
    entries: PhantomData<Descriptor>,
}

impl Table {
    pub const GUID: Guid = guid!("dcfa911d-26eb-469f-a220-38b7dc461220");

    fn entry(&self, index: usize) -> Option<&Descriptor> {
        (index < (self.number_of_entries as usize)).then(|| unsafe {
            &*(&self.entries as *const PhantomData<Descriptor> as *const Descriptor)
                .byte_add(index * (self.descriptor_size as usize))
        })
    }

    fn entries(&self) -> Vec<&Descriptor> {
        self.iter().collect()
    }

    fn iter<'a>(&'a self) -> Entries<'a> {
        Entries {
            table: self,
            index: 0,
        }
    }
}

impl Debug for Table {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Table")
            .field("version", &self.version)
            .field("number_of_entries", &self.number_of_entries)
            .field("descriptor_size", &self.descriptor_size)
            .field("flags", &self.flags)
            .field("entries", &self.entries())
            .finish()
    }
}

impl super::super::Table for Table {}

struct Entries<'a> {
    table: &'a Table,
    index: usize,
}

impl<'a> Iterator for Entries<'a> {
    type Item = &'a Descriptor;

    fn next(&mut self) -> Option<Self::Item> {
        self.table.entry(self.index).inspect(|_| self.index += 1)
    }
}
