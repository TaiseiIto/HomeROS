use super::{Char16, Handle, simple, table::Header};

/// # References
/// * [EFI_SYSTEM_TABLE](https://uefi.org/specs/UEFI/2.11/04_EFI_System_Table.html#id6)
#[repr(C)]
pub struct Table {
    hdr: Header,
    firmware_vendor: *const Char16,
    firmware_revision: u32,
    console_handle: Handle,
    con_in: *const simple::text::input::Protocol,
}
