use super::super::Node;

/// # References
/// * [Serial Attached SCSI (SAS) Extended Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#serial-attached-scsi-sas-extended-device-path)
#[derive(Debug, packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    ty: u8,
    sub_type: u8,
    length: u16,
    address: u64,
    number: u64,
    topology_info: u16,
    relative_target_port: u16,
}

impl Node for Protocol {}
