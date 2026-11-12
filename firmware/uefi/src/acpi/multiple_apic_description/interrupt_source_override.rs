use {
    super::Header,
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [Interrupt Source Override Structure](https://uefi.org/specs/ACPI/6.6/05_ACPI_Software_Programming_Model.html#interrupt-source-override-structure)
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Structure {
    header: Header,
    bus: u8,
    source: u8,
    global_system_interrupt: u32,
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
            .field("bus", &self.bus())
            .field("source", &self.source())
            .field("global_system_interrupt", &self.global_system_interrupt())
            .field("flags", &self.get_flags())
            .finish()
    }
}

impl super::Structure for Structure {}

#[io::register]
struct Flags {
    polarity: [bool; 2],
    trigger_mode: [bool; 2],
    __: [bool; 12],
}
