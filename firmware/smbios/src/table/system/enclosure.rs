use {
    super::super::baseboard_information,
    crate::{Header, header},
    alloc::string::String,
    alloc::vec::Vec,
    core::{
        fmt::{Debug, Formatter, Result},
        marker::PhantomData,
        mem::offset_of,
        slice::from_raw_parts,
    },
};

/// # References
/// * [SMBIOS Specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.10.0.pdf) 7.4 System Enclosure or Chassis (Type 3)
#[derive(packed::Accessor, table::Table)]
#[print(get_contained_elements, get_sku_number, get_rack_type, get_rack_height)]
#[repr(C, packed)]
pub struct Table {
    header: Header,
    #[string]
    manufacturer: u8,
    #[debug_type(Type)]
    typ: u8,
    #[string]
    version: u8,
    #[string]
    serial_number: u8,
    #[string]
    asset_tag_number: u8,
    #[debug_type(State)]
    boot_up_state: u8,
    #[debug_type(State)]
    power_supply_state: u8,
    #[debug_type(State)]
    thermal_state: u8,
    #[debug_type(Status)]
    security_status: u8,
    oem_defined: u32,
    height: u8,
    number_of_power_cords: u8,
    contained_element_count: u8,
    contained_element_record_length: u8,
    contained_elements: PhantomData<u8>,
}

impl Table {
    fn get_contained_elements_bytes(&self) -> Option<&[u8]> {
        let contained_element_count: usize = self.get_contained_element_count()? as usize;
        let contained_element_record_length: usize =
            self.get_contained_element_record_length()? as usize;
        let contained_element_size: usize =
            contained_element_count * contained_element_record_length;
        (offset_of!(Self, contained_elements) + contained_element_size <= self.get_header().size())
            .then(|| unsafe {
                from_raw_parts(
                    &self.contained_elements as *const PhantomData<u8> as *const u8,
                    contained_element_size,
                )
            })
            .filter(|bytes| !bytes.is_empty())
    }

    fn get_contained_elements(&self) -> Option<Vec<&ContainedElement>> {
        self.get_contained_elements_bytes()
            .map(|contained_elements| {
                contained_elements
                    .chunks(self.get_contained_element_record_length().unwrap() as usize)
                    .map(|contained_element| {
                        let contained_element: &u8 = contained_element.first().unwrap();
                        unsafe { &*(contained_element as *const u8 as *const ContainedElement) }
                    })
                    .collect()
            })
    }

    fn get_footer(&self) -> Option<&Footer> {
        Some(unsafe {
            &*((&self.contained_elements as *const PhantomData<u8>)
                .byte_add(self.get_contained_elements_bytes()?.len())
                as *const Footer)
        })
    }

    fn get_footer_offset(&self) -> Option<usize> {
        Some(offset_of!(Self, contained_elements) + self.get_contained_elements_bytes()?.len())
    }

    fn get_sku_number(&self) -> Option<String> {
        (self.get_footer_offset()? + offset_of!(Footer, sku_number) <= self.get_header().size())
            .then_some(self.get_footer()?.sku_number() as usize)
            .and_then(|sku_number| sku_number.checked_sub(1))
            .and_then(|sku_number| self.strings().get(sku_number).cloned())
    }

    fn get_rack_type(&self) -> Option<u8> {
        (self.get_footer_offset()? + offset_of!(Footer, rack_type) <= self.get_header().size())
            .then_some(self.get_footer()?.rack_type())
    }

    fn get_rack_height(&self) -> Option<u8> {
        (self.get_footer_offset()? + offset_of!(Footer, rack_height) <= self.get_header().size())
            .then_some(self.get_footer()?.rack_height())
    }
}

#[derive(packed::Accessor)]
#[repr(C, packed)]
struct Footer {
    sku_number: u8,
    rack_type: u8,
    rack_height: u8,
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

/// # References
/// * [SMBIOS Specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.10.0.pdf) 7.4.4 System Enclosure or Chassis - ContainedElements
#[derive(packed::Accessor)]
#[repr(C, packed)]
struct ContainedElement {
    typ: ContainedElementTypeRaw,
    minimum: u8,
    maximum: u8,
}

impl ContainedElement {
    fn get_type(&self) -> HeaderTypeOrBoardType {
        unsafe { self.typ.read_memory() }.into()
    }
}

impl Debug for ContainedElement {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("ContainedElement")
            .field("typ", &self.get_type())
            .field("minimum", &self.minimum())
            .field("maximum", &self.maximum())
            .finish()
    }
}

#[io::register]
struct ContainedElementType {
    typ: [bool; 7],
    select: bool,
}

#[derive(Debug)]
enum HeaderTypeOrBoardType {
    Header(header::Type),
    Board(baseboard_information::Type),
}

impl From<ContainedElementType> for HeaderTypeOrBoardType {
    fn from(contained_element_type: ContainedElementType) -> Self {
        if contained_element_type.read_select_bit() {
            Self::Header(contained_element_type.read_typ_shift().into())
        } else {
            Self::Board(contained_element_type.read_typ_shift().into())
        }
    }
}
