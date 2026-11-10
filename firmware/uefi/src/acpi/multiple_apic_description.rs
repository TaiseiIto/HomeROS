use {
    super::Header,
    core::{
        fmt::{Debug, Formatter, Result},
        marker::PhantomData,
        mem::offset_of,
    },
};

/// # References
/// * [Multiple APIC Description Table (MADT)](https://uefi.org/htmlspecs/ACPI_Spec_6_4_html/05_ACPI_Software_Programming_Model/ACPI_Software_Programming_Model.html#multiple-apic-description-table-madt)
/// # TODO
/// * Print interrupt controller structures.
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Table {
    header: Header,
    local_interrupt_controller_address: u32,
    flags: FlagsRaw,
    interrupt_controller_structure: PhantomData<u8>,
}

impl Table {
    fn get_flags(&self) -> Flags {
        self.flags().into()
    }

    fn get_interrupt_controller_structure_bytes(&self) -> &[u8] {
        &self.header.bytes()[offset_of!(Self, interrupt_controller_structure)..]
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
            .field(
                "interrupt_controller_structure",
                &self.get_interrupt_controller_structure_bytes(),
            )
            .finish()
    }
}

impl super::Table for Table {}

#[io::register]
struct Flags {
    pcat_compat: bool,
    __: [bool; 31],
}
