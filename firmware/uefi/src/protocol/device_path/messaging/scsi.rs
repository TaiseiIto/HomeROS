use super::{
    super::{Node, Type},
    SubType,
};

/// # References
/// * [SCSI Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#scsi-device-path)
#[derive(Debug)]
#[repr(C)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
    target_id: u16,
    logical_unit_number: u16,
}

impl Node for Protocol {}
