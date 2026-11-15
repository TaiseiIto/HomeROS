use {
    super::super::{
        super::{Node, Type},
        SubType,
    },
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [IPv6 Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#ipv6-device-path)
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    typ: u8,
    sub_type: u8,
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

impl Protocol {
    fn get_type(&self) -> Type {
        self.typ().into()
    }

    fn get_sub_type(&self) -> SubType {
        self.sub_type().into()
    }
}

impl Debug for Protocol {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Protocol")
            .field("typ", &self.get_type())
            .field("sub_type", &self.get_sub_type())
            .field("length", &self.length())
            .field("local_ip_address", &self.local_ip_address())
            .field("remote_ip_address", &self.remote_ip_address())
            .field("local_port", &self.local_port())
            .field("remote_port", &self.remote_port())
            .field("protocol", &self.protocol())
            .field("origin", &self.origin())
            .field("prefix_length", &self.prefix_length())
            .field("gateway", &self.gateway())
            .finish()
    }
}

impl Node for Protocol {}
