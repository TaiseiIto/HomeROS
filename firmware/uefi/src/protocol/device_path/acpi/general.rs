use super::{
    super::{Node, Type},
    SubType,
};

/// # References
/// * [ACPI Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#acpi-device-path)
#[derive(Debug, packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
    hid: u32,
    uid: u32,
}

impl Node for Protocol {}
