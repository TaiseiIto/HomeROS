use {
    super::{
        super::{Node, Type},
        SubType,
    },
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [CD-ROM Media Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#cd-rom-media-device-path)
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    typ: u8,
    sub_type: u8,
    length: u16,
    body_entry: u32,
    partition_start: u64,
    partition_size: u64,
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
            .field("body_entry", &self.body_entry())
            .field("partition_start", &self.partition_start())
            .field("partition_size", &self.partition_size())
            .finish()
    }
}

impl Node for Protocol {}
