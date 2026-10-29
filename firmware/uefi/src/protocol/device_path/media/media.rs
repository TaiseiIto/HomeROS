use {
    super::{
        super::{Node, Type},
        SubType,
    },
    crate::Guid,
};

/// # References
/// * [Media Protocol Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#media-protocol-device-path)
#[derive(Debug)]
#[repr(C, packed)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
    guid: Guid,
}

impl Node for Protocol {}
