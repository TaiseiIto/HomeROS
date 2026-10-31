use super::{
    super::{Node, Type},
    SubType,
};

/// # References
/// * [iSCSI Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#iscsi-device-path)
#[derive(Debug, packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
    protocol: u16,
    options: u16,
    number: u64,
    target_portal_group_tag: u16,
}

impl Node for Protocol {}
