use {
    super::Header,
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [I/O APIC Structure](https://uefi.org/specs/ACPI/6.6/05_ACPI_Software_Programming_Model.html#io-apic-structure)
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Structure {
    header: Header,
    id: u8,
    reserved: u8,
    address: u32,
    global_system_interrupt_base: u32,
}

impl Debug for Structure {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Structure")
            .field("header", &self.header)
            .field("id", &self.id())
            .field("address", &self.address())
            .field(
                "global_system_interrupt_base",
                &self.global_system_interrupt_base(),
            )
            .finish()
    }
}

impl super::Structure for Structure {}
