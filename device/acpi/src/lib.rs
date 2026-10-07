#![no_std]
#![feature(iter_array_chunks)]

extern crate alloc;

mod fixed_acpi_description;
pub mod root_system_description;

use core::{
    fmt::{self, Debug, Formatter},
    mem::size_of,
    num::Wrapping,
    slice::from_raw_parts,
};

/// # References
/// * [System Description Table Header](https://uefi.org/htmlspecs/ACPI_Spec_6_4_html/05_ACPI_Software_Programming_Model/ACPI_Software_Programming_Model.html#system-description-table-header)
#[repr(packed)]
pub struct Header {
    signature: [u8; 4],
    length: [u8; 4],
    revision: u8,
    checksum: u8,
    oem_id: [u8; 6],
    oem_table_id: [u8; 8],
    oem_revision: [u8; 4],
    creator_id: [u8; 4],
    creator_revision: [u8; 4],
}

impl Header {
    pub fn body(&self) -> &[u8] {
        &self.bytes()[size_of::<Self>()..]
    }

    pub fn is_correct(&self) -> bool {
        self.bytes()
            .into_iter()
            .copied()
            .map(Wrapping)
            .sum::<Wrapping<u8>>()
            .0
            == 0
    }

    pub fn table<'a>(&'a self) -> Result<&'a dyn Table, &'a str> {
        let header: *const Self = self as *const Self;
        match self.signature() {
            "RSDT" => Ok(unsafe { &*(header as *const root_system_description::Table) }),
            "FACP" => Ok(unsafe { &*(header as *const fixed_acpi_description::Table) }),
            signature => Err(signature),
        }
    }

    fn bytes(&self) -> &[u8] {
        let bytes: *const Self = self as *const Self;
        let bytes: *const u8 = bytes as *const u8;
        unsafe { from_raw_parts(bytes, self.length()) }
    }

    fn creator_id(&self) -> &str {
        str::from_utf8(&self.creator_id).unwrap()
    }

    fn length(&self) -> usize {
        u32::from_le_bytes(self.length.clone()) as usize
    }

    fn oem_id(&self) -> &str {
        str::from_utf8(&self.oem_id).unwrap()
    }

    fn oem_table_id(&self) -> &str {
        str::from_utf8(&self.oem_table_id).unwrap()
    }

    fn signature(&self) -> &str {
        str::from_utf8(&self.signature).unwrap()
    }
}

impl Debug for Header {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Header")
            .field("signature", &self.signature())
            .field("revision", &self.revision)
            .field("oem_id", &self.oem_id())
            .field("oem_table_id", &self.oem_table_id())
            .field("oem_revision", &self.oem_revision)
            .field("creator_id", &self.creator_id())
            .field("creator_revision", &self.creator_revision)
            .finish()
    }
}

trait Table: Debug {}
