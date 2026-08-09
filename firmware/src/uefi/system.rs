use super::table::Header;

/// # References
/// * [EFI_SYSTEM_TABLE](https://uefi.org/specs/UEFI/2.11/04_EFI_System_Table.html#id6)
#[repr(C)]
pub struct Table {
    hdr: Header,
}
