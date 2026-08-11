use crate::{Char16, Handle, Status, Void, protocol::DevicePath};

/// References
/// * [Exit](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-exit)
pub type Exit = extern "efiapi" fn(Handle, Status, usize, *const Char16) -> Status;

/// References
/// * [ExitBootServices](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-exitbootservices)
pub type ExitServices = extern "efiapi" fn(Handle, usize) -> Status;

/// References
/// * [LoadImage](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-loadimage)
pub type Load =
    extern "efiapi" fn(bool, Handle, *const DevicePath, *const Void, usize, *mut Handle) -> Status;

/// References
/// * [StartImage](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-startimage)
pub type Start = extern "efiapi" fn(Handle, *mut usize, *mut *const Char16) -> Status;

/// References
/// * [UnloadImage](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-unloadimage)
pub type Unload = extern "efiapi" fn(Handle) -> Status;
