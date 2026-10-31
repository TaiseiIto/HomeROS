use super::{
    super::{Node, Type},
    SubType,
};

/// # References
/// * [REST Service Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#rest-service-device-path)
/// # TODO
/// * Print all types of protocol.
#[derive(Debug, packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
    rest_service: u8,
    access_mode: u8,
}

impl Node for Protocol {}
