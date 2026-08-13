pub mod address;
pub mod map;
pub mod page;
pub mod pool;

use crate::{Status, Void};

/// # References
/// * [EFI_ALLOCATE_TYPE](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-allocatepages)
#[repr(C)]
pub enum AllocateType {
    AnyPages,
    MaxAddress,
    Address,
    MaxType,
}

/// # References
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

/// # References
/// * [CopyMem](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-copymem)
pub type Copy = extern "efiapi" fn(*mut Void, *const Void, usize) -> Status;

/// # References
/// * [SetMem](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-setmem)
pub type Set = extern "efiapi" fn(*mut Void, usize, u8) -> Status;
