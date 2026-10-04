use crate::{Guid, Void};

/// # References
/// * [EFI_CONFIGURATION_TABLE](https://uefi.org/specs/UEFI/2.11/04_EFI_System_Table.html#efi-configuration-table)
#[derive(Debug)]
#[repr(C)]
pub struct Table {
    guid: Guid,
    table: *const Void,
}
