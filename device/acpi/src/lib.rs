#![no_std]

pub mod root_system_description;

use core::fmt::{Debug, Formatter, Result};

/// # References
/// * [System Description Table Header](https://uefi.org/htmlspecs/ACPI_Spec_6_4_html/05_ACPI_Software_Programming_Model/ACPI_Software_Programming_Model.html#system-description-table-header)
#[repr(C)]
pub struct Header {
    signature: [u8; 4],
    length: u32,
    revision: u8,
    checksum: u8,
    oem_id: [u8; 6],
    oem_table_id: [u8; 8],
    oem_revision: u32,
    creator_id: [u8; 4],
    creator_revision: u32,
}

impl Header {
    fn creator_id(&self) -> &str {
        str::from_utf8(&self.creator_id).unwrap()
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
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
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
