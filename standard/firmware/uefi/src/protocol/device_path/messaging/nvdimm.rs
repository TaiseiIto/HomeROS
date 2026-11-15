use {
    super::{
        super::{Node, Type},
        SubType,
    },
    crate::Guid,
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [NVDIMM Namespace path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#nvdimm-namespace-path)
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    typ: u8,
    sub_type: u8,
    length: u16,
    uuid: Guid,
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
            .field("uuid", &self.uuid())
            .finish()
    }
}

impl Node for Protocol {}
