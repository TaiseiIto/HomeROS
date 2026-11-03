mod adr;
mod expanded;
mod general;
mod nvdimm;

use {
    super::Node,
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [ACPI Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#acpi-device-path)
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    typ: u8,
    sub_type: u8,
}

impl Protocol {
    fn get_sub_type(&self) -> SubType {
        self.sub_type().into()
    }

    fn read(&self) -> &dyn Node {
        match self.get_sub_type() {
            SubType::Acpi => unsafe { &*(self as *const Self as *const general::Protocol) },
            SubType::Expanded => unsafe { &*(self as *const Self as *const expanded::Protocol) },
            SubType::Adr => unsafe { &*(self as *const Self as *const adr::Protocol) },
            SubType::Nvdimm => unsafe { &*(self as *const Self as *const nvdimm::Protocol) },
            SubType::Unknown(_) => panic!(),
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
enum SubType {
    Acpi,
    Expanded,
    Adr,
    Nvdimm,
    Unknown(u8),
}

impl From<u8> for SubType {
    fn from(sub_type: u8) -> Self {
        match sub_type {
            1 => Self::Acpi,
            2 => Self::Expanded,
            3 => Self::Adr,
            4 => Self::Nvdimm,
            sub_type => Self::Unknown(sub_type),
        }
    }
}
