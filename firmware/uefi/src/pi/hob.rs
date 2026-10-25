use {
    crate::service::boot::memory::address::Physical,
    core::fmt::{Debug, Formatter, Result},
};

#[repr(transparent)]
pub struct List(HandOff);

impl List {
    fn iter(&self) -> TableIterator<'_> {
        TableIterator(&self.0.header)
    }
}

impl Debug for List {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter.debug_list().entries(self.iter()).finish()
    }
}

struct TableIterator<'a>(&'a Header);

impl<'a> Iterator for TableIterator<'a> {
    type Item = &'a Header;

    fn next(&mut self) -> Option<Self::Item> {
        (self.0.hob_type != Type::EndOfHobList).then(|| {
            let item: Self::Item = self.0;
            self.0 = unsafe { &*(self.0 as *const Header).byte_add(item.length as usize) };
            item
        })
    }
}

/// # References
/// * [EFI_HOB_GENERIC_HEADER](https://uefi.org/specs/PI/1.10/V3_HOB_Code_Definitions.html#efi-hob-generic-header)
#[derive(Debug)]
#[repr(C)]
struct Header {
    hob_type: Type,
    length: u16,
    reserved: u32,
}

#[derive(Debug, Eq, PartialEq)]
#[repr(u16)]
enum Type {
    Handoff = 0x0001,
    MemoryAllocation = 0x0002,
    ResourceDescriptor = 0x0003,
    GuidExtension = 0x0004,
    Fv = 0x0005,
    Cpu = 0x0006,
    MemoryPool = 0x0007,
    Fv2 = 0x0009,
    LoadPeimUnused = 0x000a,
    UefiCapsule = 0x000b,
    Fv3 = 0x000c,
    ResourceDescriptor2 = 0x000d,
    Unused = 0xfffe,
    EndOfHobList = 0xffff,
}

/// # References
/// * [EFI_HOB_HANDOFF_INFO_TABLE](https://uefi.org/specs/PI/1.10/V3_HOB_Code_Definitions.html#efi-hob-handoff-info-table-phit-hob)
#[derive(Debug)]
#[repr(C)]
struct HandOff {
    header: Header,
    version: u32,
    boot_mode: BootMode,
    memory_top: Physical,
    memory_bottom: Physical,
    free_memory_top: Physical,
    free_memory_bottom: Physical,
    end_of_hob_list: Physical,
}

type BootMode = u32;
