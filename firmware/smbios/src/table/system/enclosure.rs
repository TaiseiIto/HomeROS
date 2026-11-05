use crate::Header;

/// # References
/// * [SMBIOS Specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.10.0.pdf) 7.4 System Enclosure or Chassis (Type 3)
/// # TODO
/// * Print all elements.
#[derive(packed::Accessor, table::Table)]
#[repr(C, packed)]
pub struct Table {
    header: Header,
    #[string]
    manufacturer: u8,
    typ: u8,
    #[string]
    version: u8,
    #[string]
    serial_number: u8,
    #[string]
    asset_tag_number: u8,
    boot_up_state: u8,
    power_supply_state: u8,
    thermal_state: u8,
    security_status: u8,
    oem_defined: u32,
    height: u8,
    number_of_power_cords: u8,
    contained_element_count: u8,
    contained_element_record_length: u8,
}

/// # References
/// * [SMBIOS Specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.10.0.pdf) 7.4.1 System Enclosure or Chassis Type
#[derive(Debug)]
enum Type {
    Other,
    Unknown,
    Desktop,
    LowProfileDesktop,
    PizzaBox,
    MiniTower,
    Tower,
    Portable,
    Laptop,
    Notebook,
    HandHeld,
    DockingStation,
    AllInOne,
    SubNotebook,
    SpaceSaving,
    LunchBox,
    MainServerChassis,
    ExpansionChassis,
    SubChassis,
    BusExpansionChassis,
    PeripheralChassis,
    RaidChassis,
    RackMountChassis,
    SealedCasePc,
    MultiSystemChassis,
    CompactPci,
    AdvancedTca,
    Blade,
    BladeEnclosure,
    Tablet,
    Convertible,
    Detachable,
    IotGateway,
    EmbeddedPc,
    MiniPc,
    StickPc,
}

impl From<u8> for Type {
    fn from(typ: u8) -> Self {
        match typ {
            0x01 => Self::Other,
            0x02 => Self::Unknown,
            0x03 => Self::Desktop,
            0x04 => Self::LowProfileDesktop,
            0x05 => Self::PizzaBox,
            0x06 => Self::MiniTower,
            0x07 => Self::Tower,
            0x08 => Self::Portable,
            0x09 => Self::Laptop,
            0x0a => Self::Notebook,
            0x0b => Self::HandHeld,
            0x0c => Self::DockingStation,
            0x0d => Self::AllInOne,
            0x0e => Self::SubNotebook,
            0x0f => Self::SpaceSaving,
            0x10 => Self::LunchBox,
            0x11 => Self::MainServerChassis,
            0x12 => Self::ExpansionChassis,
            0x13 => Self::SubChassis,
            0x14 => Self::BusExpansionChassis,
            0x15 => Self::PeripheralChassis,
            0x16 => Self::RaidChassis,
            0x17 => Self::RackMountChassis,
            0x18 => Self::SealedCasePc,
            0x19 => Self::MultiSystemChassis,
            0x1a => Self::CompactPci,
            0x1b => Self::AdvancedTca,
            0x1c => Self::Blade,
            0x1d => Self::BladeEnclosure,
            0x1e => Self::Tablet,
            0x1f => Self::Convertible,
            0x20 => Self::Detachable,
            0x21 => Self::IotGateway,
            0x22 => Self::EmbeddedPc,
            0x23 => Self::MiniPc,
            0x24 => Self::StickPc,
            typ => panic!("Unknown type {:#x?}", typ),
        }
    }
}

/// # References
/// * [SMBIOS Specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.10.0.pdf) 7.4.2 System Enclosure or Chassis States
#[derive(Debug)]
enum State {
    Other,
    Unknown,
    Safe,
    Warning,
    Critical,
    NonRecoverable,
}

impl From<u8> for State {
    fn from(state: u8) -> Self {
        match state {
            0x01 => Self::Other,
            0x02 => Self::Unknown,
            0x03 => Self::Safe,
            0x04 => Self::Warning,
            0x05 => Self::Critical,
            0x06 => Self::NonRecoverable,
            state => panic!("Unknown state {:#x?}", state),
        }
    }
}

/// # References
/// * [SMBIOS Specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.10.0.pdf) 7.4.3 System Enclosure or Chassis Status
#[derive(Debug)]
enum Status {
    Other,
    Unknown,
    None,
    ExternalInterfaceLockedOut,
    ExternalInterfaceEnabled,
}

impl From<u8> for Status {
    fn from(status: u8) -> Self {
        match status {
            0x01 => Self::Other,
            0x02 => Self::Unknown,
            0x03 => Self::None,
            0x04 => Self::ExternalInterfaceLockedOut,
            0x05 => Self::ExternalInterfaceEnabled,
            status => panic!("Unknown status {:#x?}", status),
        }
    }
}
