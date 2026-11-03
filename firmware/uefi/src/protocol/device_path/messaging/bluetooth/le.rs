use super::super::super::Node;

/// # References
/// * [EFI BluetoothLE Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#efi-bluetoothle-device-path)
#[derive(Debug, packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    ty: u8,
    sub_type: u8,
    length: u16,
    address: [u8; 6],
    address_type: u8,
}

impl Node for Protocol {}
