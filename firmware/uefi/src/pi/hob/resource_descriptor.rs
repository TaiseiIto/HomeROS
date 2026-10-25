use {
    super::Header,
    crate::{Guid, service::boot::memory::address::Physical},
};

/// # References
/// * [EFI_HOB_RESOURCE_DESCRIPTOR](https://uefi.org/specs/PI/1.10/V3_HOB_Code_Definitions.html#efi-hob-resource-descriptor)
#[derive(Debug)]
#[repr(C)]
pub struct Hob {
    header: Header,
    owner: Guid,
    ty: Type,
    attribute: Attribute,
    start: Physical,
    length: u64,
}

impl super::Hob for Hob {}

/// # References
/// * [EFI_RESOURCE_TYPE](https://uefi.org/specs/PI/1.10/V3_HOB_Code_Definitions.html#efi-hob-resource-descriptor)
#[derive(Debug)]
#[repr(u32)]
enum Type {
    SystemMemory = 0x00000000,
    MemoryMappedIo = 0x00000001,
    Io = 0x00000002,
    FirmwareDevice = 0x00000003,
    MemoryMappedIoPort = 0x00000004,
    MemoryReserved = 0x00000005,
    IoReserved = 0x00000006,
    MemoryUnaccepted = 0x00000007,
    MaxMemoryType = 0x00000008,
}

/// # References
/// * [EFI_RESOURCE_ATTRIBUTE_TYPE](https://uefi.org/specs/PI/1.10/V3_HOB_Code_Definitions.html#efi-hob-resource-descriptor)
#[io::register]
struct Attribute {
    present: bool,
    initialized: bool,
    tested: bool,
    single_bit_ecc: bool,
    multiple_bit_ecc: bool,
    ecc_reserved_1: bool,
    ecc_reserved_2: bool,
    read_protected: bool,
    write_protected: bool,
    execution_protected: bool,
    uncacheable: bool,
    write_combineable: bool,
    write_through_cacheable: bool,
    write_back_cacheable: bool,
    bit16io: bool,
    bit32io: bool,
    bit64io: bool,
    uncached_exported: bool,
    read_only_protected: bool,
    read_only_protectable: bool,
    read_protectable: bool,
    write_protectable: bool,
    execution_protectable: bool,
    persistent: bool,
    persistable: bool,
    more_reliable: bool,
    encrypted: bool,
    special_purpose: bool,
    hot_pluggable: bool,
    __: [bool; 3],
}
