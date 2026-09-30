pub mod address;
pub mod map;
pub mod page;
pub mod pool;

pub use map::Map;

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
#[derive(Debug)]
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

impl From<u32> for Type {
    fn from(ty: u32) -> Self {
        match ty {
            0 => Self::Reserved,
            1 => Self::LoaderCode,
            2 => Self::LoaderData,
            3 => Self::BootServicesCode,
            4 => Self::BootServicesData,
            5 => Self::RuntimeServicesCode,
            6 => Self::RuntimeServicesData,
            7 => Self::Conventional,
            8 => Self::Unusable,
            9 => Self::ACPIReclaim,
            10 => Self::ACPIMemoryNVS,
            11 => Self::MemoryMappedIO,
            12 => Self::MemoryMappedIOPortSpace,
            13 => Self::PalCode,
            14 => Self::Persistent,
            15 => Self::Unaccepted,
            16 => Self::Max,
            _ => panic!(),
        }
    }
}

/// # References
/// * [CopyMem](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-copymem)
pub type Copy = extern "efiapi" fn(*mut Void, *const Void, usize) -> Status;

/// # References
/// * [SetMem](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-setmem)
pub type Set = extern "efiapi" fn(*mut Void, usize, u8) -> Status;
