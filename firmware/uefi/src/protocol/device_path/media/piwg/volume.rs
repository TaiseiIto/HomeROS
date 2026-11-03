use {super::super::super::Node, crate::Guid};

/// # References
/// * [PIWG Firmware Volume](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#piwg-firmware-volume)
/// * [Firmware Volume Media Device Path](https://uefi.org/specs/PI/1.10/V2_Protocols_Device_Path_Protocol.html#firmware-volume-media-device-path)
#[derive(Debug, packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    ty: u8,
    sub_type: u8,
    length: u16,
    name: Guid,
}

impl Node for Protocol {}
