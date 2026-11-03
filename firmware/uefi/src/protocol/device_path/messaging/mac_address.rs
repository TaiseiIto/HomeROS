use super::super::Node;

/// # References
/// * [MAC Address Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#mac-address-device-path)
#[derive(Debug, packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    ty: u8,
    sub_type: u8,
    length: u16,
    mac_address: [u8; 32],
    if_type: u8,
}

impl Node for Protocol {}
