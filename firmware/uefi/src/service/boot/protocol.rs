use crate::{Guid, Status, Void};

/// References
/// * [EFI_INTERFACE_TYPE](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-installprotocolinterface)
#[repr(C)]
pub enum Type {
    Native,
}

/// References
/// * [HandleProtocol](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-handleprotocol)
pub type Handle = extern "efiapi" fn(crate::Handle, *const Guid, *mut *const Void) -> Status;

/// References
/// * [InstallProtocolInterface](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-installprotocolinterface)
pub type Install =
    extern "efiapi" fn(*const crate::Handle, *const Guid, Type, *const Void) -> Status;

/// References
/// * [ReinstallProtocolInterface](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-reinstallprotocolinterface)
pub type Reinstall =
    extern "efiapi" fn(crate::Handle, *const Guid, *const Void, *const Void) -> Status;

/// References
/// * [UninstallProtocolInterface](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-uninstallprotocolinterface)
pub type Uninstall = extern "efiapi" fn(crate::Handle, *const Guid, *const Void) -> Status;
