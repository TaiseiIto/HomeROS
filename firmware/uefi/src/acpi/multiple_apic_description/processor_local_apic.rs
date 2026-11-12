use {
    super::Header,
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [Processor Local APIC Structure](https://uefi.org/specs/ACPI/6.6/05_ACPI_Software_Programming_Model.html#processor-local-apic-structure)
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Structure {
    header: Header,
    id: u8,
    flags: FlagsRaw,
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
            .field("id", &self.id())
            .field("falgs", &self.get_flags())
            .finish()
    }
}

impl super::Structure for Structure {}

#[io::register]
struct Flags {
    enabled: bool,
    online_capable: bool,
    __: [bool; 30],
}
