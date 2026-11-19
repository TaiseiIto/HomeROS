use {
    super::{
        Header,
        interrupt_source_override::{Flags, FlagsRaw},
    },
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [Local APIC NMI Structure](https://uefi.org/htmlspecs/ACPI_Spec_6_4_html/05_ACPI_Software_Programming_Model/ACPI_Software_Programming_Model.html#local-apic-nmi-structure)
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Structure {
    header: Header,
    acpi_processor_uid: u8,
    flags: FlagsRaw,
    lint: u8,
}

impl Structure {
    fn get_flags(&self) -> Flags {
        self.flags().into()
    }
}

impl Debug for Structure {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Structure")
            .field("header", &self.header)
            .field("acpi_processor_uid", &self.acpi_processor_uid())
            .field("flags", &self.get_flags())
            .field("lint", &self.lint())
            .finish()
    }
}

impl super::Structure for Structure {
    fn initialize(&mut self) {}
}
