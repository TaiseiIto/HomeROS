use crate::Status;

/// # References
/// * [EFI_TPL](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-raisetpl)
pub type Priority = usize;

/// # References
/// * [EFI_RAISE_TPL](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-raisetpl)
pub type Raise = extern "efiapi" fn(Priority) -> Status;

/// # References
/// * [EFI_RESTORE_TPL](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-restoretpl)
pub type Restore = extern "efiapi" fn(Priority) -> Status;
