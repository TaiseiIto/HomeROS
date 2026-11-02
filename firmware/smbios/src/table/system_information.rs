use {
    crate::{Header, Table as Trait},
    alloc::string::String,
    core::fmt::{Debug, Formatter, Result},
    uuid::Uuid,
};

/// # References
/// * [SMBIOS Specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.10.0.pdf) 7.2 System Information (Type 1)
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Table {
    header: Header,
    manufacturer: u8,
    product_name: u8,
    version: u8,
    serial_number: u8,
    uuid: Uuid,
    wake_up_type: u8,
    sku_number: u8,
    family: u8,
}

impl Table {
    fn get_manufacturer(&self) -> Option<String> {
        self.strings().get(self.manufacturer() as usize).cloned()
    }

    fn get_product_name(&self) -> Option<String> {
        self.strings().get(self.product_name() as usize).cloned()
    }

    fn get_version(&self) -> Option<String> {
        self.strings().get(self.version() as usize).cloned()
    }
}

impl Debug for Table {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Table")
            .field("header", &self.header)
            .field("manufacturer", &self.get_manufacturer())
            .field("product_name", &self.get_product_name())
            .field("version", &self.get_version())
            .field("serial_number", &self.serial_number())
            .field("uuid", &self.uuid())
            .field("wake_up_type", &self.wake_up_type())
            .field("sku_number", &self.sku_number())
            .field("family", &self.family())
            .finish()
    }
}

impl Trait for Table {
    fn get_header(&self) -> &Header {
        &self.header
    }
}
