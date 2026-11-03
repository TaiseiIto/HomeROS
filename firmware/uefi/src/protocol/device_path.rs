mod acpi;
mod bios_boot_specification;
mod end_of_hardware;
mod hardware;
mod media;
mod messaging;

use core::fmt::{Debug, Formatter, Result};

/// # References
/// * [EFI_DEVICE_PATH_PROTOCOL](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#efi-device-path-protocol)
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    ty: u8,
}

impl Protocol {
    fn get_type(&self) -> Type {
        self.ty().into()
    }
}

impl Protocol {
    fn read(&self) -> &dyn Node {
        match self.get_type() {
            Type::Hardware => unsafe { &*(self as *const Self as *const hardware::Protocol) },
            Type::Acpi => unsafe { &*(self as *const Self as *const acpi::Protocol) },
            Type::Messaging => unsafe { &*(self as *const Self as *const messaging::Protocol) },
            Type::Media => unsafe { &*(self as *const Self as *const media::Protocol) },
            Type::EndOfHardware => unsafe {
                &*(self as *const Self as *const end_of_hardware::Protocol)
            },
            Type::BiosBootSpecification => unsafe {
                &*(self as *const Self as *const bios_boot_specification::Protocol)
            },
            Type::Unknown(_) => panic!(),
        }
    }
}

impl Debug for Protocol {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        self.read().fmt(formatter)
    }
}

/// # References
/// * [Type](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#generic-device-path-structures)
#[derive(Clone, Copy, Debug)]
enum Type {
    Hardware,
    Acpi,
    Messaging,
    Media,
    BiosBootSpecification,
    EndOfHardware,
    Unknown(u8),
}

impl From<u8> for Type {
    fn from(typ: u8) -> Self {
        match typ {
            0x01 => Self::Hardware,
            0x02 => Self::Acpi,
            0x03 => Self::Messaging,
            0x04 => Self::Media,
            0x05 => Self::BiosBootSpecification,
            0x7f => Self::EndOfHardware,
            typ => Self::Unknown(typ),
        }
    }
}

trait Node: Debug {}
