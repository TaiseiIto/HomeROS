use {crate::Header, core::marker::PhantomData};

/// # References
/// * [SMBIOS Specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.10.0.pdf) 7.3 Baseboard (or Module) Information (Type 2)
#[derive(packed::Accessor, table::Table)]
#[repr(C, packed)]
pub struct Table {
    header: Header,
    #[string]
    manufacturer: u8,
    #[string]
    product: u8,
    #[string]
    version: u8,
    #[string]
    serial_number: u8,
    #[string]
    asset_tag: u8,
    #[debug_type(Flags)]
    feature: FlagsRaw,
    #[string]
    location_in_chassis: u8,
    chassis_handle: u16,
    #[debug_type(Type)]
    board: u8,
    number_of_contained_object_handles: u8,
    contained_object_handles: PhantomData<u16>,
}

/// # References
/// * [SMBIOS Specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.10.0.pdf) 7.3.1 Baseboard - feature flags
#[io::register]
struct Flags {
    hosting_board: bool,
    requires_daughter_board_or_auxiliary_card: bool,
    removable: bool,
    replaceable: bool,
    hot_swappable: bool,
    __: [bool; 3],
}

/// # References
/// * [SMBIOS Specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.10.0.pdf) 7.3.2 Baseboard - Board Type
#[derive(Debug)]
enum Type {
    Unknown,
    Other,
    ServerBlade,
    ConnectivitySwitch,
    SystemManagementModule,
    ProcessorModule,
    IoModule,
    MemoryModule,
    DaughterBoard,
    MotherBoard,
    ProcessorMemoryModule,
    ProcessorIoModule,
    InterconnectBoard,
}

impl From<u8> for Type {
    fn from(typ: u8) -> Self {
        match typ {
            0x01 => Self::Unknown,
            0x02 => Self::Other,
            0x03 => Self::ServerBlade,
            0x04 => Self::ConnectivitySwitch,
            0x05 => Self::SystemManagementModule,
            0x06 => Self::ProcessorModule,
            0x07 => Self::IoModule,
            0x08 => Self::MemoryModule,
            0x09 => Self::DaughterBoard,
            0x0a => Self::MotherBoard,
            0x0b => Self::ProcessorMemoryModule,
            0x0c => Self::ProcessorIoModule,
            0x0d => Self::InterconnectBoard,
            _ => panic!(),
        }
    }
}
