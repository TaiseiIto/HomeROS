use super::{
    super::{Node, Type},
    SubType,
};

/// # References
/// * [PCI Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#pci-device-path)
#[derive(Debug)]
#[repr(C)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
    function: u8,
    device: u8,
}

impl Node for Protocol {}
