use super::{
    super::{Node, Type},
    SubType,
};

/// # References
/// * [UART Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#uart-device-path)
#[derive(Debug, packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
    reserved: u32,
    baud_rate: u64,
    data_bits: u8,
    parity: u8,
    stop_bits: u8,
}

impl Node for Protocol {}
