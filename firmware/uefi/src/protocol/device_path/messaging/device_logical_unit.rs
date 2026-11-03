use super::super::Node;

/// # References
/// * [Device Logical Unit](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#device-logical-unit)
#[derive(Debug, packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    ty: u8,
    sub_type: u8,
    length: u16,
    lun: u8,
}

impl Node for Protocol {}
