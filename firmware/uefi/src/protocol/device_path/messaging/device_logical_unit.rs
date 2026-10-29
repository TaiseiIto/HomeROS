use super::{
    super::{Node, Type},
    SubType,
};

/// # References
/// * [Device Logical Unit](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#device-logical-unit)
#[derive(Debug)]
#[repr(C)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
    lun: u8,
}

impl Node for Protocol {}
