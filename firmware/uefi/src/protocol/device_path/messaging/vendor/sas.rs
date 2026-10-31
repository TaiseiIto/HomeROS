use {
    super::super::{
        super::{Node, Type},
        SubType,
    },
    crate::Guid,
    guid::guid,
};

/// # References
/// * [Serial Attached SCSI (SAS) Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#serial-attached-scsi-sas-device-path)
#[derive(Debug, packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
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
}

impl Node for Protocol {}
