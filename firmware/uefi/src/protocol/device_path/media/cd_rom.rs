use super::super::Node;

/// # References
/// * [CD-ROM Media Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#cd-rom-media-device-path)
#[derive(Debug, packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    ty: u8,
    sub_type: u8,
    length: u16,
    body_entry: u32,
    partition_start: u64,
    partition_size: u64,
}

impl Node for Protocol {}
