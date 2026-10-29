mod memory_mapped;
mod pccard;
mod pci;

use {
    super::{Node, Type},
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [Hardware Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#hardware-device-path)
/// # TODO
/// * Print all subtypes.
#[repr(C, packed)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
}

impl Protocol {
    fn read(&self) -> &dyn Node {
        match &self.sub_type {
            SubType::Pci => unsafe { &*(self as *const Self as *const pci::Protocol) },
            SubType::PcCard => unsafe { &*(self as *const Self as *const pccard::Protocol) },
            SubType::MemoryMapped => unsafe {
                &*(self as *const Self as *const memory_mapped::Protocol)
            },
            _ => panic!(),
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
/// * [Hardware Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#hardware-device-path)
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
enum SubType {
    Pci = 0x01,
    PcCard = 0x02,
    MemoryMapped = 0x03,
    Vendor = 0x04,
    Controller = 0x05,
    Bmc = 0x06,
}
