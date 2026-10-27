use crate::{Handle, Status, protocol::device_path::Protocol};

/// # References
/// * [ConnectController](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-connectcontroller)
pub type Connect = extern "efiapi" fn(Handle, *const Handle, *const Protocol, bool) -> Status;

/// # References
/// * [DisconnectController](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-disconnectcontroller)
pub type Disconnect = extern "efiapi" fn(Handle, Handle, Handle) -> Status;
