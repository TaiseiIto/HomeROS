use super::{
    super::{Node, Type},
    SubType,
};

/// # References
/// * [Controller Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#controller-device-path)
#[derive(Debug, packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
    number: u32,
}

impl Node for Protocol {}
