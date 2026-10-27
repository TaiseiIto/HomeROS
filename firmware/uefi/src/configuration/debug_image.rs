/// # References
/// * [EFI_DEBUG_IMAGE_INFO_TABLE_HEADER](https://uefi.org/specs/UEFI/2.11/18_Protocols_Debugger_Support.html#efi-image-info)
#[derive(Debug)]
#[repr(C)]
pub struct Table {
    update_status: UpdateStatusRaw,
    table_size: u32,
    info: &'static Info,
}

#[io::register]
struct UpdateStatus {
    update_in_progress: bool,
    modified: bool,
    __: [bool; 30],
}

/// # References
/// * [EFI_DEBUG_IMAGE_INFO](https://uefi.org/specs/UEFI/2.11/18_Protocols_Debugger_Support.html#efi-image-info)
#[derive(Debug)]
#[repr(C)]
struct Info {
    ty: &'static u32,
    normal: &'static Normal,
}

/// # References
/// * [EFI_DEBUG_IMAGE_INFO_NORMAL](https://uefi.org/specs/UEFI/2.11/18_Protocols_Debugger_Support.html#efi-image-info)
#[derive(Debug)]
#[repr(C)]
struct Normal {
    ty: u32,
}
