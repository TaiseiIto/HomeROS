mod acpi;

use {
    super::{Node, Type},
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [ACPI Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#acpi-device-path)
/// # TODO
/// * Print all subtypes.
#[repr(C)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
}

impl Protocol {
    fn read(&self) -> &dyn Node {
        match &self.sub_type {
            SubType::Acpi => unsafe { &*(self as *const Self as *const acpi::Protocol) },
            _ => unimplemented!(),
        }
    }
}

impl Debug for Protocol {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        self.read().fmt(formatter)
    }
}

impl Node for Protocol {}

/// # References
/// * [ACPI Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#acpi-device-path)
#[derive(Debug)]
#[repr(u8)]
enum SubType {
    Acpi = 1,
    ExpandedAcpi = 2,
    Adr = 3,
    Nvdimm = 4,
}
