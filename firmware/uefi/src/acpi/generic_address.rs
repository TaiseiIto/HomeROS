use core::fmt::{Debug, Formatter, Result};

/// # References
/// * [Generic Address Structure](https://uefi.org/specs/ACPI/6.6/05_ACPI_Software_Programming_Model.html#generic-address-structure-gas)
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Structure {
    space_id: u8,
    bit_width: u8,
    bit_offset: u8,
    access_size: u8,
    address: u64,
}

impl Structure {
    fn get_space_id(&self) -> SpaceId {
        self.space_id().into()
    }
}

impl Debug for Structure {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Structure")
            .field("space_id", &self.get_space_id())
            .field("bit_width", &self.bit_width())
            .field("bit_offset", &self.bit_offset())
            .field("access_size", &self.access_size())
            .field("address", &self.address())
            .finish()
    }
}

#[derive(Debug)]
enum SpaceId {
    SystemMemorySpace,
    SystemIoSpace,
    PciConfigurationSpace,
    EmbeddedController,
    SmBus,
    SystemCmos,
    PciBarTarget,
    Ipmi,
    GeneralPurposeIo,
    GenericSerialBus,
    PlatformCommunicationsChannel,
    PlatformRuntimeMechanism,
    Reserved(u8),
    FunctionalFixedHardware,
    OedDefined(u8),
}

impl From<u8> for SpaceId {
    fn from(space_id: u8) -> Self {
        match space_id {
            0x00 => Self::SystemMemorySpace,
            0x01 => Self::SystemIoSpace,
            0x02 => Self::PciConfigurationSpace,
            0x03 => Self::EmbeddedController,
            0x04 => Self::SmBus,
            0x05 => Self::SystemCmos,
            0x06 => Self::PciBarTarget,
            0x07 => Self::Ipmi,
            0x08 => Self::GeneralPurposeIo,
            0x09 => Self::GenericSerialBus,
            0x0a => Self::PlatformCommunicationsChannel,
            0x0b => Self::PlatformRuntimeMechanism,
            0x0c..=0x7e => Self::Reserved(space_id),
            0x7f => Self::FunctionalFixedHardware,
            0x80..=0xff => Self::OedDefined(space_id),
        }
    }
}
