use {
    super::super::{
        super::{Node, Type},
        SubType,
    },
    crate::Guid,
};

/// # References
/// * [Firmware File Media Device Path](https://uefi.org/specs/PI/1.10/V2_Protocols_Device_Path_Protocol.html#firmware-file-media-device-path)
#[derive(Debug)]
#[repr(C)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
    name: Guid,
}

impl Node for Protocol {}
