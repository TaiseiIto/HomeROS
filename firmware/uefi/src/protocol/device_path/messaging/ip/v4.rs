use super::super::{
    super::{Node, Type},
    SubType,
};

/// # References
/// * [IPv4 Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#ipv4-device-path)
#[derive(Debug, packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
    local_ip_address: u32,
    remote_ip_address: u32,
    local_port: u16,
    remote_port: u16,
    protocol: u16,
    static_ip_address: u8,
    gateway_ip_address: u32,
    subnet_mask: u32,
}

impl Node for Protocol {}
