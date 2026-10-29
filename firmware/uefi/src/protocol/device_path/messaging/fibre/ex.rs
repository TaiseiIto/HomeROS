use super::super::{
    super::{Node, Type},
    SubType,
};

/// # References
/// * [Fibre Channel Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#fibre-channel-device-path)
#[derive(Debug)]
#[repr(C)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
    reserved: u32,
    world_wide_name: u64,
    logical_unit_number: u64,
}

impl Node for Protocol {}
