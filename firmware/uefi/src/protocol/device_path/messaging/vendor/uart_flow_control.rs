use {
    super::super::{
        super::{Node, Type},
        SubType,
    },
    crate::Guid,
    guid::guid,
};

/// # References
/// * [UART Flow Control Message Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#uart-flow-control-messaging-path)
#[derive(packed::Accessor, Debug)]
#[repr(C, packed)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
    guid: Guid,
    flow_control_map: u32,
}

impl Protocol {
    pub const GUID: Guid = guid!("37499a9d-542f-4c89-a026-35da142094e4");
}

impl Node for Protocol {}
