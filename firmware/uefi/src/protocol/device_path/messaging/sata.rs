use super::{
    super::{Node, Type},
    SubType,
};

/// # References
/// * [SATA Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#sata-device-path)
#[derive(Debug, packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
    hba_port_number: u16,
    port_multiplier_port_number: u16,
    logical_unit_number: u16,
}

impl Node for Protocol {}
