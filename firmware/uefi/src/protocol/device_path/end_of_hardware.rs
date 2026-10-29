use super::{Node, Type};

#[derive(Debug, packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
}

impl Node for Protocol {}

/// # References
/// * [Generic Device Path Structures](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#generic-device-path-structures)
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
enum SubType {
    Entire = 0xff,
    Instance = 0x01,
}
