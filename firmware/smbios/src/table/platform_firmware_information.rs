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
    #[debug_type(Characteristics)]
    characteristics: CharacteristicsRaw,
    characteristics_extension_bytes: u16,
    major_release: u8,
    minor_release: u8,
    embedded_controller_major_release: u8,
    embedded_controller_minor_release: u8,
    extended_rom_size: u16,
}

/// # References
/// * [SMBIOS Specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.10.0.pdf) 7.1.1 Firmware Characteristics
#[io::register]
struct Characteristics {
    __: [bool; 2],
    unknown: bool,
    not_supported: bool,
    isa_is_supported: bool,
    mca_is_supported: bool,
    eisa_is_supported: bool,
    pci_is_supported: bool,
    pcmcia_is_supported: bool,
    plug_and_play_is_supported: bool,
    apm_is_supported: bool,
    upgradeable: bool,
    shadowing_is_allowed: bool,
    vl_vesa_is_supported: bool,
    escd_support_is_available: bool,
    boot_from_cd_is_supported: bool,
    selectable_boot_is_supported: bool,
    firmware_rom_is_socketed: bool,
    boot_from_pcmcia_is_supported: bool,
    edd_specification_is_supported: bool,
    nec_9800_is_supported: bool,
    toshiba_is_supported: bool,
    floppy_360_kb_services_are_supported: bool,
    floppy_1_2_mb_services_are_supported: bool,
    floppy_720_kb_services_are_supported: bool,
    floppy_2_88_mb_services_are_supported: bool,
    print_screen_service_is_supported: bool,
    keyboard_8042_services_are_supported: bool,
    serial_services_are_supported: bool,
    printer_services_are_supported: bool,
    video_services_are_supported: bool,
    nec_pc_98: bool,
    platform_firmware_vendor: u16,
    system_vendor: u16,
}
