use {
    super::{AllocateType, Type, address::Physical},
    crate::Status,
};

/// References
/// * [AllocatePages](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-allocatepages)
pub type Allocate = extern "efiapi" fn(AllocateType, Type, usize, *mut Physical) -> Status;
