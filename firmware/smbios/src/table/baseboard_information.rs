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
    feature_flags: u8,
    #[string]
    location_in_chassis: u8,
    chassis_handle: u16,
    board_type: u8,
    number_of_contained_object_handles: u8,
    contained_object_handles: PhantomData<u16>,
}
