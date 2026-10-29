use super::{Node, Type};

/// # References
/// * [Hardware Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#hardware-device-path)
#[derive(Debug)]
#[repr(C)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
}

impl Node for Protocol {}

/// # References
/// * [Hardware Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#hardware-device-path)
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
enum SubType {
    Pci = 0x01,
    PcCard = 0x02,
    MemoryMapped = 0x03,
    Vendor = 0x04,
    Controller = 0x05,
    Bmc = 0x06,
}
