use crate::{Guid, Handle, Status, Void};

/// References
/// * [InstallProtocolInterface](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-installprotocolinterface)
pub type Install = extern "efiapi" fn(*const Handle, *const Guid, Type, *const Void) -> Status;

/// References
/// * [EFI_INTERFACE_TYPE](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-installprotocolinterface)
#[repr(C)]
pub enum Type {
    Native,
}
