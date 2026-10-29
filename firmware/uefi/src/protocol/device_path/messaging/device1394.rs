use super::{
    super::{Node, Type},
    SubType,
};

/// # References
/// * [1394 Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#device-path)
#[derive(Debug, packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
    reserved: u32,
    guid: u64,
}

impl Node for Protocol {}
