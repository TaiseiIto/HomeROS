use {
    super::{
        super::{Node, Type},
        SubType,
    },
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [Relative Offset Range](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#relative-offset-range)
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    typ: u8,
    sub_type: u8,
    length: u16,
    reserved: u32,
    starting_offset: u64,
    ending_offset: u64,
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
            .field("reserved", &self.reserved())
            .field("starting_offset", &self.starting_offset())
            .field("ending_offset", &self.ending_offset())
            .finish()
    }
}

impl Node for Protocol {}
