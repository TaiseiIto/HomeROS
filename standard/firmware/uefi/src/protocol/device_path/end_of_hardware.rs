use {
    super::{Node, Type},
    core::fmt::{Debug, Formatter, Result},
};

#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    typ: u8,
    sub_type: u8,
    length: u16,
}

impl Protocol {
    fn get_type(&self) -> Type {
        self.typ().into()
    }

    fn get_sub_type(&self) -> SubType {
        self.sub_type().into()
    }
}

impl Debug for Protocol {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Protocol")
            .field("typ", &self.get_type())
            .field("sub_type", &self.get_sub_type())
            .field("length", &self.length())
            .finish()
    }
}

impl Node for Protocol {}

/// # References
/// * [Generic Device Path Structures](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#generic-device-path-structures)
#[derive(Clone, Copy, Debug)]
enum SubType {
    Entire,
    Instance,
}

impl From<u8> for SubType {
    fn from(sub_type: u8) -> Self {
        match sub_type {
            0xff => Self::Entire,
            0x01 => Self::Instance,
            sub_type => panic!("Unknown type {:#x?}", sub_type),
        }
    }
}
