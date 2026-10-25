use crate::service::boot::memory::address::Physical;

/// # References
/// * [EFI_HOB_GENERIC_HEADER](https://uefi.org/specs/PI/1.10/V3_HOB_Code_Definitions.html#efi-hob-generic-header)
#[derive(Debug)]
#[repr(C)]
pub struct Header {
    hob_type: Type,
    length: u16,
    reserved: u32,
}

#[derive(Debug)]
#[repr(u32)]
pub enum Type {
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
pub struct HandOff {
    header: Header,
    version: u32,
    boot_mode: BootMode,
    memory_top: Physical,
    memory_bottom: Physical,
    free_memory_top: Physical,
    free_memory_bottom: Physical,
    end_of_hob_list: Physical,
}

pub type BootMode = u32;
