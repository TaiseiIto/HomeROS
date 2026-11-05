use {crate::Header, uuid::Uuid};

/// # References
/// * [SMBIOS Specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.10.0.pdf) 7.2 System Information (Type 1)
#[derive(packed::Accessor, table::Table)]
#[repr(C, packed)]
pub struct Table {
    header: Header,
    #[string]
    manufacturer: u8,
    #[string]
    product_name: u8,
    #[string]
    version: u8,
    #[string]
    serial_number: u8,
    uuid: Uuid,
    wake_up_type: u8,
    #[string]
    sku_number: u8,
    #[string]
    family: u8,
}

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
            wake_up => panic!("Unknown wake up {:#x?}", wake_up),
        }
    }
}
