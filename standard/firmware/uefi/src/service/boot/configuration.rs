use crate::{Guid, Status, Void};

/// # References
/// * [InstallConfigurationTable](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-installconfigurationtable)
pub type Install = extern "efiapi" fn(*const Guid, *const Void) -> Status;
