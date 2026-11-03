use {
    super::{super::Node, SubType},
    crate::Guid,
};

/// # References
/// * [Media Protocol Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#media-protocol-device-path)
#[derive(Debug, packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    ty: u8,
    sub_type: SubType,
    length: u16,
    guid: Guid,
}

impl Node for Protocol {}
