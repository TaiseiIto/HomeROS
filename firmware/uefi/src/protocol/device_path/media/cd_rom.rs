use super::{
    super::{Node, Type},
    SubType,
};

/// # References
/// * [CD-ROM Media Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#cd-rom-media-device-path)
#[derive(Debug)]
#[repr(C)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
    body_entry: u32,
    partition_start: u64,
    partition_size: u64,
}

impl Node for Protocol {}
