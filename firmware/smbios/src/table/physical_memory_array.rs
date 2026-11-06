use crate::Header;

/// # References
/// * [SMBIOS Specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.10.0.pdf) 7.17 Physical Memory Array (Type 16)
#[derive(packed::Accessor, table::Table)]
#[repr(C, packed)]
pub struct Table {
    header: Header,
    #[debug_type(Location)]
    location: u8,
    #[debug_type(Use)]
    us: u8,
    #[debug_type(Correction)]
    memory_error_correction: u8,
    maximum_capacity: u32,
    memory_error_information_handle: u16,
    number_of_memory_devices: u16,
    extended_maximum_capacity: u64,
}

/// # References
/// * [SMBIOS Specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.10.0.pdf) 7.17.1 Memory Array - Location
#[derive(Debug)]
enum Location {
    Other,
    Unknown,
    SystemBoardOrMotherBoard,
    IsaAddOnCard,
    EisaAddOnCard,
    PciAddOnCard,
    McaAddOnCard,
    PcmciaAddOnCard,
    ProprietaryAddOnCard,
    NuBus,
    Pc98c20AddOnCard,
    Pc98c24AddOnCard,
    Pc98eAddOnCard,
    Pc98localBusAddOnCard,
    CxlAddOnCard,
}

impl From<u8> for Location {
    fn from(location: u8) -> Self {
        match location {
            0x01 => Self::Other,
            0x02 => Self::Unknown,
            0x03 => Self::SystemBoardOrMotherBoard,
            0x04 => Self::IsaAddOnCard,
            0x05 => Self::EisaAddOnCard,
            0x06 => Self::PciAddOnCard,
            0x07 => Self::McaAddOnCard,
            0x08 => Self::PcmciaAddOnCard,
            0x09 => Self::ProprietaryAddOnCard,
            0x0a => Self::NuBus,
            0xa0 => Self::Pc98c20AddOnCard,
            0xa1 => Self::Pc98c24AddOnCard,
            0xa2 => Self::Pc98eAddOnCard,
            0xa3 => Self::Pc98localBusAddOnCard,
            0xa4 => Self::CxlAddOnCard,
            _ => panic!(),
        }
    }
}

/// # References
/// * [SMBIOS Specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.10.0.pdf) 7.17.2 Memory Array - Use
#[derive(Debug)]
enum Use {
    Other,
    Unknown,
    SystemMemory,
    VideoMemory,
    FlashMemory,
    NonVolatileRam,
    CacheMemory,
}

impl From<u8> for Use {
    fn from(us: u8) -> Self {
        match us {
            0x01 => Self::Other,
            0x02 => Self::Unknown,
            0x03 => Self::SystemMemory,
            0x04 => Self::VideoMemory,
            0x05 => Self::FlashMemory,
            0x06 => Self::NonVolatileRam,
            0x07 => Self::CacheMemory,
            _ => panic!(),
        }
    }
}

/// # References
/// * [SMBIOS Specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.10.0.pdf) 7.17.3 Memory Array - Error Correction Types
#[derive(Debug)]
enum Correction {
    Other,
    Unknown,
    None,
    Parity,
    SingleBitEcc,
    MultiBitEcc,
    Crc,
}

impl From<u8> for Correction {
    fn from(correction: u8) -> Self {
        match correction {
            0x01 => Self::Other,
            0x02 => Self::Unknown,
            0x03 => Self::None,
            0x04 => Self::Parity,
            0x05 => Self::SingleBitEcc,
            0x06 => Self::MultiBitEcc,
            0x07 => Self::Crc,
            _ => panic!(),
        }
    }
}
