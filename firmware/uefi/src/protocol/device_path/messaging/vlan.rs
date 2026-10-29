use super::{
    super::{Node, Type},
    SubType,
};

/// # References
/// * [VLAN device path node](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#vlan-device-path-node)
#[derive(Debug, packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
    id: u16,
}

impl Node for Protocol {}
