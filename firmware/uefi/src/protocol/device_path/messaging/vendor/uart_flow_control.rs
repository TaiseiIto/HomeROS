use {
    super::super::super::Node,
    crate::{Guid, guid},
};

/// # References
/// * [UART Flow Control Message Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#uart-flow-control-messaging-path)
#[derive(packed::Accessor, Debug)]
#[repr(C, packed)]
pub struct Protocol {
    ty: u8,
    sub_type: u8,
    length: u16,
    guid: Guid,
    flow_control_map: u32,
}

impl Protocol {
    pub const GUID: Guid = guid!("37499a9d-542f-4c89-a026-35da142094e4");
}

impl Node for Protocol {}
