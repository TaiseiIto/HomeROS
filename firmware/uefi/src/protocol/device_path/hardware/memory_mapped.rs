use super::{
    super::{Node, Type},
    SubType,
};

/// # References
/// * [Memory Mapped Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#memory-mapped-device-path)
#[derive(Debug)]
#[repr(C, packed)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
    memory_type: u32,
    start_address: u64,
    end_address: u64,
}

impl Node for Protocol {}
