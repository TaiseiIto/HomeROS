pub mod le;

use super::super::Node;

/// # References
/// * [EFI Bluetooth Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#efi-bluetooth-device-path)
#[derive(Debug, packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    ty: u8,
    sub_type: u8,
    length: u16,
    address: [u8; 6],
}

impl Node for Protocol {}
