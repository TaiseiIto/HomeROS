use crate::Header;

/// # References
/// * [SMBIOS Specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.10.0.pdf) 7.1 Platform Firmware Information (Type 0)
/// # TODO
/// * Print bit fields.
#[derive(packed::Accessor, table::Table)]
#[repr(C, packed)]
pub struct Table {
    header: Header,
    #[string]
    vendor: u8,
    #[string]
    version: u8,
    bios_starting_address_segment: u16,
    #[string]
    release_date: u8,
    rom_size: u8,
    characteristics: u64,
    characteristics_extension_bytes: u16,
    major_release: u8,
    minor_release: u8,
    embedded_controller_major_release: u8,
    embedded_controller_minor_release: u8,
    extended_rom_size: u16,
}
