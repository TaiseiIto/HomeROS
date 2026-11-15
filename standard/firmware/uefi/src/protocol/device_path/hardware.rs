mod bmc;
mod controller;
mod memory_mapped;
mod pccard;
mod pci;
mod vendor;

use {
    super::Node,
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [Hardware Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#hardware-device-path)
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
            SubType::Pci => unsafe { &*(self as *const Self as *const pci::Protocol) },
            SubType::PcCard => unsafe { &*(self as *const Self as *const pccard::Protocol) },
            SubType::MemoryMapped => unsafe {
                &*(self as *const Self as *const memory_mapped::Protocol)
            },
            SubType::Vendor => unsafe { &*(self as *const Self as *const vendor::Protocol) },
            SubType::Controller => unsafe {
                &*(self as *const Self as *const controller::Protocol)
            },
            SubType::Bmc => unsafe { &*(self as *const Self as *const bmc::Protocol) },
            SubType::Unknown(typ) => panic!("Unknown type {:#x?}", typ),
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
enum SubType {
    Pci,
    PcCard,
    MemoryMapped,
    Vendor,
    Controller,
    Bmc,
    Unknown(u8),
}

impl From<u8> for SubType {
    fn from(sub_type: u8) -> Self {
        match sub_type {
            0x01 => Self::Pci,
            0x02 => Self::PcCard,
            0x03 => Self::MemoryMapped,
            0x04 => Self::Vendor,
            0x05 => Self::Controller,
            0x06 => Self::Bmc,
            sub_type => Self::Unknown(sub_type),
        }
    }
}
