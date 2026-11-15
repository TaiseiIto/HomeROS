use {
    super::{
        super::{Node, Type},
        SubType,
    },
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [Hard Drive](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#hard-drive)
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    typ: u8,
    sub_type: u8,
    length: u16,
    partition_nubmer: u32,
    partition_start: u64,
    partition_size: u64,
    partition_signature: u128,
    partition_format: u8,
    signature_type: u8,
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
            .field("partition_nubmer", &self.partition_nubmer())
            .field("partition_start", &self.partition_start())
            .field("partition_size", &self.partition_size())
            .field("partition_signature", &self.partition_signature())
            .field("partition_format", &self.partition_format())
            .field("signature_type", &self.signature_type())
            .finish()
    }
}

impl Node for Protocol {}
