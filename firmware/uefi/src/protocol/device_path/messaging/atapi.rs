use super::{
    super::{Node, Type},
    SubType,
};

/// # References
/// * [ATAPI Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#atapi-device-path)
#[derive(Debug, packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
    primary_secondary: u8,
    slave_master: u8,
    logical_unit_number: u16,
}

impl Node for Protocol {}
