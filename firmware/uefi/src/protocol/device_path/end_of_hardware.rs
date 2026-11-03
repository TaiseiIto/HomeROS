use super::Node;

#[derive(Debug, packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    ty: u8,
    sub_type: u8,
    length: u16,
}

impl Node for Protocol {}

/// # References
/// * [Generic Device Path Structures](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#generic-device-path-structures)
#[derive(Clone, Copy, Debug)]
enum SubType {
    Entire,
    Instance,
    Unknown(u8),
}

impl From<u8> for SubType {
    fn from(sub_type: u8) -> Self {
        match sub_type {
            0xff => Self::Entire,
            0x01 => Self::Instance,
            sub_type => Self::Unknown(sub_type),
        }
    }
}
