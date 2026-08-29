use {
    crate::{
        Char16, Handle, configuration,
        protocol::text::{input, output},
        service::{boot, runtime},
        table::Header,
    },
    core::{
        fmt::{Debug, Formatter, Result},
        slice::from_raw_parts,
    },
};

/// # References
/// * [EFI_SYSTEM_TABLE](https://uefi.org/specs/UEFI/2.11/04_EFI_System_Table.html#id6)
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

    fn configuration_tables(&self) -> &[configuration::Table] {
        unsafe { from_raw_parts(self.configuration_table, self.number_of_table_entries) }
    }
}

impl Debug for Table {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Table")
            .field("hdr", &self.hdr)
            .field("firmware_vendor", &self.firmware_vendor)
            .field("firmware_revision", &self.firmware_revision)
            .field("console_in_handle", &self.console_in_handle)
            .field("con_in", unsafe { &*self.con_in })
            .field("console_out_handle", &self.console_out_handle)
            .field("con_out", unsafe { &*self.con_out })
            .field("standard_error_handle", &self.standard_error_handle)
            .field("std_err", unsafe { &*self.std_err })
            .field("runtime_services", unsafe { &*self.runtime_services })
            .field("boot_services", unsafe { &*self.boot_services })
            .field("configuration_table", &self.configuration_tables())
            .finish()
    }
}
