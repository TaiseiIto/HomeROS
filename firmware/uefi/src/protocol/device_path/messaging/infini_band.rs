use {
    super::{
        super::{Node, Type},
        SubType,
    },
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [InfiniBand Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#infiniband-device-path)
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    typ: u8,
    sub_type: u8,
    length: u16,
    resource_flags: u32,
    port_gid: u128,
    ioc_guid_service_id: u64,
    target_port_id: u64,
    device_id: u64,
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
            .field("resource_flags", &self.resource_flags())
            .field("port_gid", &self.port_gid())
            .field("ioc_guid_service_id", &self.ioc_guid_service_id())
            .field("target_port_id", &self.target_port_id())
            .field("device_id", &self.device_id())
            .finish()
    }
}

impl Node for Protocol {}
