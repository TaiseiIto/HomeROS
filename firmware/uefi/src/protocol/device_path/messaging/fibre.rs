pub mod ex;

use super::super::Node;

/// # References
/// * [Fibre Channel Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#fibre-channel-device-path)
#[derive(Debug, packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    ty: u8,
    sub_type: u8,
    length: u16,
    reserved: u32,
    world_wide_name: u64,
    logical_unit_number: u64,
}

impl Node for Protocol {}
