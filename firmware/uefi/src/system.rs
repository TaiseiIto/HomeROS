use crate::{Char16, Handle, protocol, service, table};

/// # References
/// * [EFI_SYSTEM_TABLE](https://uefi.org/specs/UEFI/2.11/04_EFI_System_Table.html#id6)
#[repr(C)]
pub struct Table {
    hdr: table::Header,
    firmware_vendor: *const Char16,
    firmware_revision: u32,
    console_in_handle: Handle,
    con_in: *const protocol::text::Input,
    console_out_handle: Handle,
    con_out: *const protocol::text::Output,
    standard_error_handle: Handle,
    std_err: *const protocol::text::Output,
    runtime_services: *const service::runtime::Table,
}
