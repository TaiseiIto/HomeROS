pub mod address;
pub mod map;
pub mod page;
pub mod pool;

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
    Reserved,
    LoaderCode,
    LoaderData,
    BootServicesCode,
    BootServicesData,
    RuntimeServicesCode,
    RuntimeServicesData,
    Conventional,
    Unusable,
    ACPIReclaim,
    ACPIMemoryNVS,
    MemoryMappedIO,
    MemoryMappedIOPortSpace,
    PalCode,
    Persistent,
    Unaccepted,
    Max,
}
