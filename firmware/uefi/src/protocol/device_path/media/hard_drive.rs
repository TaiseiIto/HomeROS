use super::super::Node;

/// # References
/// * [Hard Drive](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#hard-drive)
#[derive(Debug, packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    ty: u8,
    sub_type: u8,
    length: u16,
    partition_nubmer: u32,
    partition_start: u64,
    partition_size: u64,
    partition_signature: u128,
    partition_format: u8,
    signature_type: u8,
}

impl Node for Protocol {}
