mod acpi;
mod bios_boot_specification;
mod end_of_hardware;
mod hardware;
mod media;
mod messaging;

use core::fmt::{Debug, Formatter, Result};

/// # References
/// * [EFI_DEVICE_PATH_PROTOCOL](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#efi-device-path-protocol)
/// # TODO
/// * Print all type.
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    ty: Type,
}

impl Protocol {
    fn read(&self) -> &dyn Node {
        match self.ty() {
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
