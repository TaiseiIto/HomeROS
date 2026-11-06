use crate::Header;

/// # References
/// * [SMBIOS Specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.10.0.pdf) 7.5 Processor Information (Type 4)
/// # TODO
/// * Print enum.
/// * Print bit field.
#[derive(packed::Accessor, table::Table)]
#[repr(C, packed)]
pub struct Table {
    header: Header,
    #[string]
    socket_designation: u8,
    #[debug_type(Type)]
    typ: u8,
    family: u8,
    #[string]
    manufacturer: u8,
    id: u64,
    #[string]
    version: u8,
    voltage: u8,
    external_clock: u16,
    max_speed: u16,
    current_speed: u16,
    status: u8,
    upgrade: u8,
    l1_cache_handle: u16,
    l2_cache_handle: u16,
    l3_cache_handle: u16,
    #[string]
    serial_number: u8,
    #[string]
    asset_tag: u8,
    #[string]
    part_number: u8,
    core_count: u8,
    core_enabled: u8,
    thread_count: u8,
    characteristics: u16,
    family2: u16,
    core_count2: u16,
    core_enabled2: u16,
    thread_count2: u16,
    thread_enabled: u16,
    #[string]
    socket_type: u8,
}

/// # References
/// * [SMBIOS Specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.10.0.pdf) 7.5.1 Processor Information - Processor Type
#[derive(Debug)]
enum Type {
    Other,
    Unknown,
    Central,
    Math,
    Dsp,
    Video,
}

impl From<u8> for Type {
    fn from(typ: u8) -> Self {
        match typ {
            0x01 => Self::Other,
            0x02 => Self::Unknown,
            0x03 => Self::Central,
            0x04 => Self::Math,
            0x05 => Self::Dsp,
            0x06 => Self::Video,
            _ => panic!(),
        }
    }
}
