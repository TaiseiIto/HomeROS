use crate::{Handle, Status};

/// References
/// * [InstallMultipleProtocolInterfaces](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-installmultipleprotocolinterfaces)
#[must_use]
pub type Install = extern "efiapi" fn(*mut Handle, ...) -> Status;

/// References
/// * [UninstallMultipleProtocolInterfaces]()
#[must_use]
pub type Uninstall = extern "efiapi" fn(Handle, ...) -> Status;
