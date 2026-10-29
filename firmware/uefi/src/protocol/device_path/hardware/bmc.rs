use super::{
    super::{Node, Type},
    SubType,
};

/// # References
/// * [BMC Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#bmc-device-path)
#[derive(Debug, packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
    interface: u8,
    base_address: u64,
}

impl Node for Protocol {}
