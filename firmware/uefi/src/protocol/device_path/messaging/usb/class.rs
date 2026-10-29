use super::super::{
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
    vendor_id: u16,
    product_id: u16,
    device_class: u8,
    device_subclass: u8,
    device_protocol: u8,
}

impl Node for Protocol {}
