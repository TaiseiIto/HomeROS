use {
    super::super::{
        super::{Node, Type},
        SubType,
    },
    crate::{Guid, guid},
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [Serial Attached SCSI (SAS) Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#serial-attached-scsi-sas-device-path)
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    typ: u8,
    sub_type: u8,
    length: u16,
    guid: Guid,
    reserved: u32,
    address: u64,
    number: u64,
    topology_info: u16,
    relative_target_port: u16,
}

impl Protocol {
    pub const GUID: Guid = guid!("d487ddb4-008b-11d9-afdc-001083ffca4d");

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
            .field("guid", &self.guid())
            .field("reserved", &self.reserved())
            .field("address", &self.address())
            .field("number", &self.number())
            .field("topology_info", &self.topology_info())
            .field("relative_target_port", &self.relative_target_port())
            .finish()
    }
}

impl Node for Protocol {}
