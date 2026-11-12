use {
    super::Header,
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [Boot Graphics Resource Table (BGRT)](https://uefi.org/specs/ACPI/6.6/05_ACPI_Software_Programming_Model.html#boot-graphics-resource-table-bgrt)
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Table {
    header: Header,
    version: u16,
    status: u8,
    image_type: u8,
    image_address: u64,
    image_offset_x: u32,
    image_offset_y: u32,
}

impl Debug for Table {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Table")
            .field("header", &self.header)
            .field("version", &self.version())
            .field("status", &self.status())
            .field("image_type", &self.image_type())
            .field("image_address", &self.image_address())
            .field("image_offset_x", &self.image_offset_x())
            .field("image_offset_y", &self.image_offset_y())
            .finish()
    }
}

impl super::Table for Table {}
