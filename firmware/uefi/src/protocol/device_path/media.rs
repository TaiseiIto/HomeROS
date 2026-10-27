use super::{Node, Type};

/// # References
/// * [Media Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#media-device-path)
#[derive(Debug)]
#[repr(C)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
}

impl Node for Protocol {}

/// # References
/// * [Media Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#media-device-path)
#[derive(Debug)]
#[repr(u8)]
enum SubType {
    HardDrive = 0x01,
    CdRom = 0x02,
    Vendor = 0x03,
    FilePath = 0x04,
    Media = 0x05,
    PiwgFile = 0x06,
    PiwgVolume = 0x07,
    RelativeOffset = 0x08,
    RamDisk = 0x09,
}
