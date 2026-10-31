use super::{
    super::{Node, Type},
    SubType,
};

/// # References
/// * [EFI Bluetooth Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#efi-bluetooth-device-path)
#[derive(Debug, packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
    address: [u8; 6],
}

impl Node for Protocol {}
