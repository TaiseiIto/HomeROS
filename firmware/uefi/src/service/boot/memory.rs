pub mod address;
pub mod page;

use address::{Physical, Virtual};

/// References
/// * [EFI_ALLOCATE_TYPE](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-allocatepages)
#[repr(C)]
pub enum AllocateType {
    AnyPages,
    MaxAddress,
    Address,
    MaxType,
}

/// References
/// * [EFI_MEMORY_TYPE](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-allocatepages)
#[repr(C)]
pub enum Type {
    ReservedMemoryType,
    LoaderCode,
    LoaderData,
    BootServicesCode,
    BootServicesData,
    RuntimeServicesCode,
    RuntimeServicesData,
    ConventionalMemory,
    UnusableMemory,
    ACPIReclaimMemory,
    ACPIMemoryNVS,
    MemoryMappedIO,
    MemoryMappedIOPortSpace,
    PalCode,
    PersistentMemory,
    UnacceptedMemoryType,
    MaxMemoryType,
}

/// References
/// * [EFI_MEMORY_DESCRIPTOR](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-getmemorymap)
#[repr(C)]
pub struct Descriptor {
    memory_type: u32,
    physical_start: Physical,
    virtual_start: Virtual,
    number_of_pages: u64,
    attribute: u64,
}
