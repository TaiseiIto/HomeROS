mod adr;
mod expanded;
mod general;
mod nvdimm;

use {
    super::{Node, Type},
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [ACPI Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#acpi-device-path)
/// # TODO
/// * Print all subtypes.
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
}

impl Protocol {
    fn read(&self) -> &dyn Node {
        match self.sub_type() {
            SubType::Acpi => unsafe { &*(self as *const Self as *const general::Protocol) },
            SubType::Expanded => unsafe { &*(self as *const Self as *const expanded::Protocol) },
            SubType::Adr => unsafe { &*(self as *const Self as *const adr::Protocol) },
            SubType::Nvdimm => unsafe { &*(self as *const Self as *const nvdimm::Protocol) },
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
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
enum SubType {
    Acpi = 1,
    Expanded = 2,
    Adr = 3,
    Nvdimm = 4,
}
