use {
    super::generic_address,
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [IA-PC HPET (High Precision Event Timer) Specification](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/software-developers-hpet-spec-1-0a.pdf) 3.2.4 The ACPI 2.0 HPET Description Table (HPET)
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Table {
    header: super::Header,
    block_id: BlockIdRaw,
    base_address: generic_address::Structure,
    number: u8,
    minimum_clock_tick: u16,
    page_protection: u8,
}

impl Table {
    fn get_block_id(&self) -> BlockId {
        self.block_id().into()
    }
}

impl Debug for Table {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Table")
            .field("header", &self.header)
            .field("block_id", &self.get_block_id())
            .field("base_address", &self.base_address())
            .field("number", &self.number())
            .field("minimum_clock_tick", &self.minimum_clock_tick())
            .field("page_protection", &self.page_protection())
            .finish()
    }
}

impl super::Table for Table {}

#[io::register]
struct BlockId {
    hardware_rev_id: u8,
    number_of_comparators: [bool; 5],
    count_size_cap: bool,
    __: bool,
    legacy_replacement_irp_routing_capable: bool,
    pci_vendor_id: u16,
}
