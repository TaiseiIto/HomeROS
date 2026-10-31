use super::super::{
    super::{Node, Type},
    SubType,
};

/// # References
/// * [EFI BluetoothLE Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#efi-bluetoothle-device-path)
#[derive(Debug, packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
    address: [u8; 6],
    address_type: u8,
}

impl Node for Protocol {}
