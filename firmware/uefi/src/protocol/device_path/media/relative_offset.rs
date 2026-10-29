use super::{
    super::{Node, Type},
    SubType,
};

/// # References
/// * [Relative Offset Range](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#relative-offset-range)
#[derive(Debug)]
#[repr(C, packed)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
    reserved: u32,
    starting_offset: u64,
    ending_offset: u64,
}

impl Node for Protocol {}
