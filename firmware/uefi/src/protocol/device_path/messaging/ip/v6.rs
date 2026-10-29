use super::super::{
    super::{Node, Type},
    SubType,
};

/// # References
/// * [IPv6 Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#ipv6-device-path)
#[derive(Debug, packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
    local_ip_address: u128,
    remote_ip_address: u128,
    local_port: u16,
    remote_port: u16,
    protocol: u16,
    origin: u8,
    prefix_length: u8,
    gateway: u128,
}

impl Node for Protocol {}
