//! # References
//! * [SMBIOS Specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.10.0.pdf)
//! # TODO
//! * Implement SMBIOS tables.

#![no_std]

extern crate alloc;

use {
    alloc::string::String,
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [SMBIOS Specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.10.0.pdf) 5.2.1 SMBIOS 2.1 (32-bit) Entry Point
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct EntryPoint32 {
    anchor: [u8; 4],
    checksum: u8,
    length: u8,
    major_version: u8,
    minor_version: u8,
    max_size: u16,
    revision: u8,
    formatted_area: [u8; 5],
    intermediate_anchor: [u8; 5],
    internediate_checksum: u8,
    structure_table_length: u16,
    structure_table_address: u32,
    number_of_smbios_structures: u16,
    smbios_bcd_revision: u8,
}

impl EntryPoint32 {
    fn anchor_string(&self) -> String {
        String::from_utf8(self.anchor().to_vec()).unwrap()
    }

    fn intermediate_anchor_string(&self) -> String {
        String::from_utf8(self.intermediate_anchor().to_vec()).unwrap()
    }
}

impl Debug for EntryPoint32 {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("EntryPoint32")
            .field("anchor", &self.anchor_string())
            .field("checksum", &self.checksum())
            .field("length", &self.length())
            .field("major_version", &self.major_version())
            .field("minor_version", &self.minor_version())
            .field("max_size", &self.max_size())
            .field("revision", &self.revision())
            .field("formatted_area", &self.formatted_area())
            .field("intermediate_anchor", &self.intermediate_anchor_string())
            .field("internediate_checksum", &self.internediate_checksum())
            .field("structure_table_length", &self.structure_table_length())
            .field("structure_table_address", &self.structure_table_address())
            .field(
                "number_of_smbios_structures",
                &self.number_of_smbios_structures(),
            )
            .field("smbios_bcd_revision", &self.smbios_bcd_revision())
            .finish()
    }
}
