use crate::Header;

/// # References
/// * [SMBIOS Specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.10.0.pdf) 7.17 Physical Memory Array (Type 16)
#[derive(packed::Accessor, table::Table)]
#[repr(C, packed)]
pub struct Table {
    header: Header,
    location: u8,
    us: u8,
    memory_error_correction: u8,
    maximum_capacity: u32,
    memory_error_information_handle: u16,
    number_of_memory_devices: u16,
    extended_maximum_capacity: u64,
}
