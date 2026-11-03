use super::super::Node;

/// # References
/// * [SD (Secure Digital) Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#sd-secure-digital-device-path)
#[derive(Debug, packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    ty: u8,
    sub_type: u8,
    length: u16,
    slot_number: u8,
}

impl Node for Protocol {}
