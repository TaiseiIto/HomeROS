use crate::{
    Char16, Handle, configuration,
    protocol::text::{input, output},
    service::{boot, runtime},
    table::Header,
};

/// # References
/// * [EFI_SYSTEM_TABLE](https://uefi.org/specs/UEFI/2.11/04_EFI_System_Table.html#id6)
#[derive(Debug)]
#[repr(C)]
pub struct Table {
    hdr: Header,
    firmware_vendor: *const Char16,
    firmware_revision: u32,
    console_in_handle: Handle,
    con_in: *const input::Functions,
    console_out_handle: Handle,
    con_out: *const output::Functions,
    standard_error_handle: Handle,
    std_err: *const output::Functions,
    runtime_services: *const runtime::Table,
    boot_services: *const boot::Table,
    number_of_table_entries: usize,
    configuration_table: *const configuration::Table,
}

impl Table {
    pub fn write(&self, string: &str) {
        unsafe { &*self.con_out }.write_string(string);
    }
}
