use {
    super::super::{
        super::{Node, Type},
        SubType,
    },
    crate::Guid,
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [PIWG Firmware File](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#piwg-firmware-file)
/// * [Firmware File Media Device Path](https://uefi.org/specs/PI/1.10/V2_Protocols_Device_Path_Protocol.html#firmware-file-media-device-path)
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    typ: u8,
    sub_type: u8,
    length: u16,
    name: Guid,
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
            .field("name", &self.name())
            .finish()
    }
}

impl Node for Protocol {}
