use {
    crate::{Header, Table as Trait},
    alloc::string::String,
    core::{
        fmt::{Debug, Formatter, Result},
        marker::PhantomData,
        slice::from_raw_parts,
    },
};

/// # References
/// * [SMBIOS Specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.10.0.pdf) 7.4 System Enclosure or Chassis (Type 3)
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Table {
    header: Header,
    manufacturer: u8,
    typ: u8,
    version: u8,
    serial_number: u8,
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
    contained_elements: PhantomData<u8>,
}

impl Table {
    fn get_manufacturer(&self) -> Option<String> {
        self.manufacturer_index()
            .and_then(|index| self.strings().get(index).cloned())
    }

    fn manufacturer_index(&self) -> Option<usize> {
        (self.manufacturer() as usize).checked_sub(1)
    }

    fn get_type(&self) -> Type {
        self.typ().into()
    }

    fn get_version(&self) -> Option<String> {
        self.version_index()
            .and_then(|index| self.strings().get(index).cloned())
    }

    fn version_index(&self) -> Option<usize> {
        (self.version() as usize).checked_sub(1)
    }

    fn get_serial_number(&self) -> Option<String> {
        self.serial_number_index()
            .and_then(|index| self.strings().get(index).cloned())
    }

    fn serial_number_index(&self) -> Option<usize> {
        (self.serial_number() as usize).checked_sub(1)
    }

    fn get_asset_tag_number(&self) -> Option<String> {
        self.asset_tag_number_index()
            .and_then(|index| self.strings().get(index).cloned())
    }

    fn asset_tag_number_index(&self) -> Option<usize> {
        (self.asset_tag_number() as usize).checked_sub(1)
    }

    fn get_boot_up_state(&self) -> State {
        self.boot_up_state().into()
    }

    fn get_power_supply_state(&self) -> State {
        self.power_supply_state().into()
    }

    fn get_thermal_state(&self) -> State {
        self.thermal_state().into()
    }

    fn get_security_status(&self) -> Status {
        self.security_status().into()
    }

    fn get_sku_number(&self) -> Option<String> {
        self.sku_number_index()
            .and_then(|index| self.strings().get(index).cloned())
    }

    fn sku_number_index(&self) -> Option<usize> {
        (self.footer().sku_number as usize).checked_sub(1)
    }

    fn get_contained_elements(&self) -> &[u8] {
        unsafe {
            from_raw_parts(
                &self.contained_elements as *const PhantomData<u8> as *const u8,
                (self.contained_element_count() * self.contained_element_record_length()) as usize,
            )
        }
    }

    fn footer(&self) -> &Footer {
        unsafe {
            &*(&self.contained_elements as *const PhantomData<u8> as *const Footer)
                .byte_add(self.get_contained_elements().len())
        }
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
            .field("type", &self.get_type())
            .field("version", &self.get_version())
            .field("serial_number", &self.get_serial_number())
            .field("asset_tag_number", &self.get_asset_tag_number())
            .field("boot_up_state", &self.get_boot_up_state())
            .field("power_supply_state", &self.get_power_supply_state())
            .field("thermal_state", &self.get_thermal_state())
            .field("security_status", &self.get_security_status())
            .field("oem_defined", &self.oem_defined())
            .field("height", &self.height())
            .field("number_of_power_cords", &self.number_of_power_cords())
            .field("contained_element_count", &self.contained_element_count())
            .field(
                "contained_element_record_length",
                &self.contained_element_record_length(),
            )
            .field("contained_elements", &self.get_contained_elements())
            .field("sku_number", &self.get_sku_number())
            .field("rack_type", &self.footer().rack_type)
            .field("rack_height", &self.footer().rack_height)
            .finish()
    }
}

impl Trait for Table {}

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
    Unimplemented(u8),
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
            typ => Self::Unimplemented(typ),
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
    Unimplemented(u8),
}

impl From<u8> for State {
    fn from(typ: u8) -> Self {
        match typ {
            0x01 => Self::Other,
            0x02 => Self::Unknown,
            0x03 => Self::Safe,
            0x04 => Self::Warning,
            0x05 => Self::Critical,
            0x06 => Self::NonRecoverable,
            typ => Self::Unimplemented(typ),
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
    Unimplemented(u8),
}

impl From<u8> for Status {
    fn from(typ: u8) -> Self {
        match typ {
            0x01 => Self::Other,
            0x02 => Self::Unknown,
            0x03 => Self::None,
            0x04 => Self::ExternalInterfaceLockedOut,
            0x05 => Self::ExternalInterfaceEnabled,
            typ => Self::Unimplemented(typ),
        }
    }
}

#[derive(packed::Accessor, Debug)]
#[repr(C, packed)]
struct Footer {
    sku_number: u8,
    rack_type: u8,
    rack_height: u8,
}
