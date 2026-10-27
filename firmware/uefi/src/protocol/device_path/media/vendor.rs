use {
    super::{
        super::{Node, Type},
        SubType,
    },
    crate::Guid,
};

/// # References
/// * [Vendor-Devined Media Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#cd-rom-media-device-path)
#[derive(Debug)]
#[repr(C)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
    guid: Guid,
}

impl Node for Protocol {}
