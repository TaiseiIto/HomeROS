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
        self.manufacturer_index()
            .and_then(|index| self.strings().get(index).cloned())
    }

    fn manufacturer_index(&self) -> Option<usize> {
        (self.manufacturer() as usize).checked_sub(1)
    }

    fn get_product_name(&self) -> Option<String> {
        self.product_name_index()
            .and_then(|index| self.strings().get(index).cloned())
    }

    fn product_name_index(&self) -> Option<usize> {
        (self.product_name() as usize).checked_sub(1)
    }

    fn get_version(&self) -> Option<String> {
        self.version_index()
            .and_then(|index| self.strings().get(index).cloned())
    }

    fn version_index(&self) -> Option<usize> {
        (self.version() as usize).checked_sub(1)
    }

    fn get_wake_up_type(&self) -> WakeUp {
        self.wake_up_type().into()
    }

    fn get_sku_number(&self) -> Option<String> {
        self.sku_number_index()
            .and_then(|index| self.strings().get(index).cloned())
    }

    fn sku_number_index(&self) -> Option<usize> {
        (self.sku_number() as usize).checked_sub(1)
    }

    fn get_family(&self) -> Option<String> {
        self.family_index()
            .and_then(|index| self.strings().get(index).cloned())
    }

    fn family_index(&self) -> Option<usize> {
        (self.family() as usize).checked_sub(1)
    }
}

impl Debug for Table {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Table")
            .field("type", &self.header.get_type())
            .field("length", &self.header.length())
            .field("handle", &self.header.handle())
            .field("manufacturer", &self.get_manufacturer())
            .field("product_name", &self.get_product_name())
            .field("version", &self.get_version())
            .field("serial_number", &self.serial_number())
            .field("uuid", &self.uuid())
            .field("wake_up_type", &self.get_wake_up_type())
            .field("sku_number", &self.get_sku_number())
            .field("family", &self.get_family())
            .finish()
    }
}

impl Trait for Table {}

/// # References
/// * [SMBIOS Specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.10.0.pdf) 7.2.2 System - Wake-up Type
#[derive(Debug)]
enum WakeUp {
    Reserved,
    Other,
    Unknown,
    ApmTimer,
    ModemRing,
    LanRemote,
    PowerSwitch,
    PciPme,
    AcPowerRestored,
    Unimplemented(u8),
}

impl From<u8> for WakeUp {
    fn from(wake_up: u8) -> Self {
        match wake_up {
            0x00 => Self::Reserved,
            0x01 => Self::Other,
            0x02 => Self::Unknown,
            0x03 => Self::ApmTimer,
            0x04 => Self::ModemRing,
            0x05 => Self::LanRemote,
            0x06 => Self::PowerSwitch,
            0x07 => Self::PciPme,
            0x08 => Self::AcPowerRestored,
            wake_up => Self::Unimplemented(wake_up),
        }
    }
}
