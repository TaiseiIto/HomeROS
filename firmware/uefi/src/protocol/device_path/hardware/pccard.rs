use super::{
    super::{Node, Type},
    SubType,
};

/// # References
/// * [PCCARD Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#pccard-device-path)
#[derive(Debug, packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
    function: u8,
}

impl Node for Protocol {}
