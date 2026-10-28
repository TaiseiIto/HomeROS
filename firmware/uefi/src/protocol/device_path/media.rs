mod cd_rom;
mod file_path;
mod hard_drive;
mod media;
mod piwg;
mod ram_disk;
mod relative_offset;
mod vendor;

use {
    super::{Node, Type},
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [Media Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#media-device-path)
#[repr(C)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
}

impl Protocol {
    fn read(&self) -> &dyn Node {
        match &self.sub_type {
            SubType::HardDrive => unsafe { &*(self as *const Self as *const hard_drive::Protocol) },
            SubType::CdRom => unsafe { &*(self as *const Self as *const cd_rom::Protocol) },
            SubType::Vendor => unsafe { &*(self as *const Self as *const vendor::Protocol) },
            SubType::FilePath => unsafe { &*(self as *const Self as *const file_path::Protocol) },
            SubType::Media => unsafe { &*(self as *const Self as *const media::Protocol) },
            SubType::PiwgFile => unsafe { &*(self as *const Self as *const piwg::file::Protocol) },
            SubType::PiwgVolume => unsafe {
                &*(self as *const Self as *const piwg::volume::Protocol)
            },
            SubType::RelativeOffset => unsafe {
                &*(self as *const Self as *const relative_offset::Protocol)
            },
            SubType::RamDisk => unsafe { &*(self as *const Self as *const ram_disk::Protocol) },
        }
    }
}

impl Debug for Protocol {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
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
