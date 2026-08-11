use crate::{Handle, Status, protocol::DevicePath};

/// References
/// * [ConnectController](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-connectcontroller)
#[must_use]
pub type Connect = extern "efiapi" fn(Handle, *const Handle, *const DevicePath, bool) -> Status;

/// References
/// * [DisconnectController](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-disconnectcontroller)
#[must_use]
pub type Disconnect = extern "efiapi" fn(Handle, Handle, Handle) -> Status;
