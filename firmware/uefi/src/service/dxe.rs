use crate::table;

/// # References
/// * [DXE_SERVICES](https://uefi.org/specs/PI/1.8/V2_UEFI_System_Table.html#dxe-services)
#[derive(Debug)]
#[repr(C)]
pub struct Table {
    hdr: table::Header,
}
