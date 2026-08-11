use {
    super::Type,
    crate::{Status, Void},
};

/// References
/// * [AllocatePool](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-allocatepool)
pub type Allocate = extern "efiapi" fn(Type, usize, *mut *mut Void) -> Status;

/// References
/// * [FreePool](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-freepool)
pub type Free = extern "efiapi" fn(*const Void) -> Status;
