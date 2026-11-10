use {
    super::Header,
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [Multiple APIC Description Table (MADT)](https://uefi.org/htmlspecs/ACPI_Spec_6_4_html/05_ACPI_Software_Programming_Model/ACPI_Software_Programming_Model.html#multiple-apic-description-table-madt)
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Table {
    header: Header,
    local_interrupt_controller_address: u32,
    flags: FlagsRaw,
}

impl Table {
    fn get_flags(&self) -> Flags {
        self.flags().into()
    }
}

impl Debug for Table {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Table")
            .field("header", &self.header)
            .field(
                "local_interrupt_controller_address",
                &self.local_interrupt_controller_address(),
            )
            .field("flags", &self.get_flags())
            .finish()
    }
}

impl super::Table for Table {}

#[io::register]
struct Flags {
    pcat_compat: bool,
    __: [bool; 31],
}
