mod cd_rom;
mod file_path;
mod guid;
mod hard_drive;
mod piwg;
mod ram_disk;
mod relative_offset;
mod vendor;

use {
    super::Node,
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [Media Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#media-device-path)
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    typ: u8,
    sub_type: u8,
}

impl Protocol {
    fn get_sub_type(&self) -> SubType {
        self.sub_type().into()
    }

    fn read(&self) -> &dyn Node {
        match self.get_sub_type() {
            SubType::HardDrive => unsafe { &*(self as *const Self as *const hard_drive::Protocol) },
            SubType::CdRom => unsafe { &*(self as *const Self as *const cd_rom::Protocol) },
            SubType::Vendor => unsafe { &*(self as *const Self as *const vendor::Protocol) },
            SubType::FilePath => unsafe { &*(self as *const Self as *const file_path::Protocol) },
            SubType::Media => unsafe { &*(self as *const Self as *const guid::Protocol) },
            SubType::PiwgFile => unsafe { &*(self as *const Self as *const piwg::file::Protocol) },
            SubType::PiwgVolume => unsafe {
                &*(self as *const Self as *const piwg::volume::Protocol)
            },
            SubType::RelativeOffset => unsafe {
                &*(self as *const Self as *const relative_offset::Protocol)
            },
            SubType::RamDisk => unsafe { &*(self as *const Self as *const ram_disk::Protocol) },
            SubType::Unknown(_) => panic!(),
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
#[derive(Clone, Copy, Debug)]
enum SubType {
    HardDrive,
    CdRom,
    Vendor,
    FilePath,
    Media,
    PiwgFile,
    PiwgVolume,
    RelativeOffset,
    RamDisk,
    Unknown(u8),
}

impl From<u8> for SubType {
    fn from(sub_type: u8) -> Self {
        match sub_type {
            0x01 => Self::HardDrive,
            0x02 => Self::CdRom,
            0x03 => Self::Vendor,
            0x04 => Self::FilePath,
            0x05 => Self::Media,
            0x06 => Self::PiwgFile,
            0x07 => Self::PiwgVolume,
            0x08 => Self::RelativeOffset,
            0x09 => Self::RamDisk,
            sub_type => Self::Unknown(sub_type),
        }
    }
}
