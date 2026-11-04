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
            .field("header", &self.header())
            .field("socket_designation", &self.socket_designation())
            .field("typ", &self.typ())
            .field("family", &self.family())
            .field("manufacturer", &self.manufacturer())
            .field("id", &self.id())
            .field("version", &self.version())
            .field("voltage", &self.voltage())
            .field("external_clock", &self.external_clock())
            .field("max_speed", &self.max_speed())
            .field("current_speed", &self.current_speed())
            .field("status", &self.status())
            .field("upgrade", &self.upgrade())
            .field("l1_cache_handle", &self.l1_cache_handle())
            .field("l2_cache_handle", &self.l2_cache_handle())
            .field("l3_cache_handle", &self.l3_cache_handle())
            .field("serial_number", &self.serial_number())
            .field("asset_tag", &self.asset_tag())
            .field("part_number", &self.part_number())
            .field("core_count", &self.core_count())
            .field("core_enabled", &self.core_enabled())
            .field("thread_count", &self.thread_count())
            .field("characteristics", &self.characteristics())
            .field("family2", &self.family2())
            .field("core_count2", &self.core_count2())
            .field("core_enabled2", &self.core_enabled2())
            .field("thread_count2", &self.thread_count2())
            .field("thread_enabled", &self.thread_enabled())
            .field("socket_type", &self.socket_type())
            .finish()
    }
}

impl Trait for Table {}
