use {
    super::super::{
        super::{Node, Type},
        SubType,
    },
    crate::{Guid, guid},
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [UART Flow Control Message Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#uart-flow-control-messaging-path)
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    typ: u8,
    sub_type: u8,
    length: u16,
    guid: Guid,
    flow_control_map: u32,
}

impl Protocol {
    pub const GUID: Guid = guid!("37499a9d-542f-4c89-a026-35da142094e4");

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
            .field("guid", &self.guid())
            .field("flow_control_map", &self.flow_control_map())
            .finish()
    }
}

impl Node for Protocol {}
