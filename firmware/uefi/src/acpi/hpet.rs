use core::fmt::{Debug, Formatter, Result};

/// # References
/// * [IA-PC HPET (High Precision Event Timer) Specification](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/software-developers-hpet-spec-1-0a.pdf) 3.2.4 The ACPI 2.0 HPET Description Table (HPET)
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Table {
    header: super::Header,
    event_timer_block_id: u32,
    base_address: [u8; 12],
    number: u8,
    minimum_clock_tick: u16,
    page_protection: u8,
}

impl Debug for Table {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Table")
            .field("header", &self.header)
            .field("event_timer_block_id", &self.event_timer_block_id())
            .field("base_address", &self.base_address())
            .field("number", &self.number())
            .field("minimum_clock_tick", &self.minimum_clock_tick())
            .field("page_protection", &self.page_protection())
            .finish()
    }
}

impl super::Table for Table {}
