use super::{
    super::{Node, Type},
    SubType,
};

/// # References
/// * [I2O Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#i-2-o-device-path)
#[derive(Debug)]
#[repr(C)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
    tid: u32,
}

impl Node for Protocol {}
