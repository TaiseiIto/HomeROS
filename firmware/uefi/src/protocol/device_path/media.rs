mod hard_drive;
mod piwg;

use {
    super::{Node, Type},
    core::fmt::{self, Debug, Formatter},
};

/// # References
/// * [Media Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#media-device-path)
/// # TODO
/// * Print all sub type.
#[repr(C)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
}

impl Protocol {
    fn read(&self) -> Result<&dyn Node, &SubType> {
        match &self.sub_type {
            SubType::HardDrive => {
                Ok(unsafe { &*(self as *const Self as *const hard_drive::Protocol) })
            }
            SubType::PiwgFile => {
                Ok(unsafe { &*(self as *const Self as *const piwg::file::Protocol) })
            }
            sub_type => Err(sub_type),
        }
    }
}

impl Debug for Protocol {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        self.read().fmt(formatter)
    }
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
