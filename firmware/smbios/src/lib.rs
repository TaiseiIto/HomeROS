//! # References
//! * [SMBIOS Specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.10.0.pdf)
//! # TODO
//! * Implement SMBIOS tables.

#![no_std]

extern crate alloc;

mod header;
mod strings;

use {
    alloc::{string::String, vec::Vec},
    core::fmt::{Debug, Formatter, Result},
    header::Header,
    strings::Strings,
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
    table_length: u16,
    table_address: u32,
    number_of_structures: u16,
    bcd_revision: u8,
}

impl EntryPoint32 {
    fn anchor_string(&self) -> String {
        String::from_utf8(self.anchor().to_vec()).unwrap()
    }

    fn intermediate_anchor_string(&self) -> String {
        String::from_utf8(self.intermediate_anchor().to_vec()).unwrap()
    }
}

impl EntryPoint for EntryPoint32 {
    fn first_header(&self) -> usize {
        self.table_address() as usize
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
            .field("table_length", &self.table_length())
            .field("table_address", &self.table_address())
            .field("number_of_structures", &self.number_of_structures())
            .field("bcd_revision", &self.bcd_revision())
            .field("headers", &self.headers())
            .finish()
    }
}

/// # References
/// * [SMBIOS Specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.10.0.pdf) 5.2.2 SMBIOS 3.0 (64-bit) Entry Point
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct EntryPoint64 {
    anchor: [u8; 5],
    checksum: u8,
    length: u8,
    major_version: u8,
    minor_version: u8,
    docrev: u8,
    revision: u8,
    reserved: u8,
    max_size: u32,
    table_address: u64,
}

impl EntryPoint64 {
    fn anchor_string(&self) -> String {
        String::from_utf8(self.anchor().to_vec()).unwrap()
    }
}

impl EntryPoint for EntryPoint64 {
    fn first_header(&self) -> usize {
        self.table_address() as usize
    }
}

impl Debug for EntryPoint64 {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("EntryPoint64")
            .field("anchor", &self.anchor_string())
            .field("checksum", &self.checksum())
            .field("length", &self.length())
            .field("major_version", &self.major_version())
            .field("minor_version", &self.minor_version())
            .field("docrev", &self.docrev())
            .field("revision", &self.revision())
            .field("reserved", &self.reserved())
            .field("max_size", &self.max_size())
            .field("table_address", &self.table_address())
            .field("headers", &self.headers())
            .finish()
    }
}

trait EntryPoint {
    fn first_header(&self) -> usize;

    fn headers(&self) -> Vec<&Header> {
        let list: header::List = unsafe { &*(self.first_header() as *const Header) }.into();
        list.collect()
    }
}
