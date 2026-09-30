use {
    super::{AllocateType, Type, address::Physical},
    crate::Status,
    unit::prefix::KIBI,
};

/// # References
/// * [AllocatePages](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-allocatepages)
pub type Allocate = extern "efiapi" fn(AllocateType, Type, usize, *mut Physical) -> Status;

/// # References
/// * [FreePages](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-freepages)
pub type Free = extern "efiapi" fn(Physical, usize) -> Status;

pub const SIZE: usize = 4 * (KIBI as usize);
