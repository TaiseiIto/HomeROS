use super::{
    super::{Node, Type},
    SubType,
};

/// # References
/// * [USB Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#usb-device-paths)
#[derive(Debug)]
#[repr(C)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
    parent_port_number: u8,
    interface: u8,
}

impl Node for Protocol {}
