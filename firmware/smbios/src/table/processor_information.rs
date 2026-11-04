use {
    crate::{Header, Table as Trait},
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [SMBIOS Specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.10.0.pdf) 7.5 Processor Information (Type 4)
#[derive(packed::Accessor, table::Table)]
#[repr(C, packed)]
pub struct Table {
    header: Header,
    #[string]
    socket_designation: u8,
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

impl Debug for Table {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Table")
            .field("header", &self.get_header())
            .field("socket_designation", &self.get_socket_designation())
            .field("typ", &self.get_typ())
            .field("family", &self.get_family())
            .field("manufacturer", &self.get_manufacturer())
            .field("id", &self.get_id())
            .field("version", &self.get_version())
            .field("voltage", &self.get_voltage())
            .field("external_clock", &self.get_external_clock())
            .field("max_speed", &self.get_max_speed())
            .field("current_speed", &self.get_current_speed())
            .field("status", &self.get_status())
            .field("upgrade", &self.get_upgrade())
            .field("l1_cache_handle", &self.get_l1_cache_handle())
            .field("l2_cache_handle", &self.get_l2_cache_handle())
            .field("l3_cache_handle", &self.get_l3_cache_handle())
            .field("serial_number", &self.get_serial_number())
            .field("asset_tag", &self.get_asset_tag())
            .field("part_number", &self.get_part_number())
            .field("core_count", &self.get_core_count())
            .field("core_enabled", &self.get_core_enabled())
            .field("thread_count", &self.get_thread_count())
            .field("characteristics", &self.get_characteristics())
            .field("family2", &self.get_family2())
            .field("core_count2", &self.get_core_count2())
            .field("core_enabled2", &self.get_core_enabled2())
            .field("thread_count2", &self.get_thread_count2())
            .field("thread_enabled", &self.get_thread_enabled())
            .field("socket_type", &self.get_socket_type())
            .finish()
    }
}

impl Trait for Table {}
