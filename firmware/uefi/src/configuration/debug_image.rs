/// # References
/// * [EFI_DEBUG_IMAGE_INFO_TABLE_HEADER](https://uefi.org/specs/UEFI/2.11/18_Protocols_Debugger_Support.html#efi-image-info)
#[derive(Debug)]
#[repr(C)]
pub struct Table {
    update_status: u32,
    table_size: u32,
}
