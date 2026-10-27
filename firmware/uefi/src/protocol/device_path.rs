mod end_of_hardware;
mod hardware;
mod media;

use core::fmt::{self, Debug, Formatter};

/// # References
/// * [EFI_DEVICE_PATH_PROTOCOL](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#efi-device-path-protocol)
/// # TODO
/// * Print all type.
#[repr(C)]
pub struct Protocol {
    ty: Type,
}

impl Protocol {
    fn read(&self) -> &dyn Node {
        match &self.ty {
            Type::Hardware => unsafe { &*(self as *const Self as *const hardware::Protocol) },
            Type::EndOfHardware => unsafe {
                &*(self as *const Self as *const end_of_hardware::Protocol)
            },
            Type::Media => unsafe { &*(self as *const Self as *const media::Protocol) },
            _ => panic!(),
        }
    }
}

impl Debug for Protocol {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        self.read().fmt(formatter)
    }
}

/// # References
/// * [Type](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#generic-device-path-structures)
#[derive(Debug)]
#[repr(u8)]
enum Type {
    Hardware = 0x01,
    Acpi = 0x02,
    Messaging = 0x03,
    Media = 0x04,
    BiosBootSpecification = 0x05,
    EndOfHardware = 0x7f,
}

trait Node: Debug {}
