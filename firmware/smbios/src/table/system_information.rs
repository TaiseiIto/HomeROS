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
    wake_up_type: WakeUp,
    sku_number: u8,
    family: u8,
}

impl Table {
    fn get_manufacturer(&self) -> Option<String> {
        self.manufacturer_index()
            .and_then(|index| self.strings().get(index).cloned())
    }

    fn get_product_name(&self) -> Option<String> {
        self.product_name_index()
            .and_then(|index| self.strings().get(index).cloned())
    }

    fn get_version(&self) -> Option<String> {
        self.version_index()
            .and_then(|index| self.strings().get(index).cloned())
    }

    fn get_sku_number(&self) -> Option<String> {
        self.sku_number_index()
            .and_then(|index| self.strings().get(index).cloned())
    }

    fn get_family(&self) -> Option<String> {
        self.family_index()
            .and_then(|index| self.strings().get(index).cloned())
    }

    fn manufacturer_index(&self) -> Option<usize> {
        (self.manufacturer() as usize).checked_sub(1)
    }

    fn product_name_index(&self) -> Option<usize> {
        (self.product_name() as usize).checked_sub(1)
    }

    fn version_index(&self) -> Option<usize> {
        (self.version() as usize).checked_sub(1)
    }

    fn sku_number_index(&self) -> Option<usize> {
        (self.sku_number() as usize).checked_sub(1)
    }

    fn family_index(&self) -> Option<usize> {
        (self.family() as usize).checked_sub(1)
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
            .field("sku_number", &self.get_sku_number())
            .field("family", &self.get_family())
            .finish()
    }
}

impl Trait for Table {
    fn get_header(&self) -> &Header {
        &self.header
    }
}

/// # References
/// * [SMBIOS Specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.10.0.pdf) 7.2.2 System - Wake-up Type
#[derive(Debug)]
#[repr(u8)]
enum WakeUp {
    Reserved = 0x00,
    Other = 0x01,
    Unknown = 0x02,
    ApmTimer = 0x03,
    ModemRing = 0x04,
    LanRemote = 0x05,
    PowerSwitch = 0x06,
    PciPme = 0x07,
    AcPowerRestored = 0x08,
}
