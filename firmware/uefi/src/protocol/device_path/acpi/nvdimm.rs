use super::{
    super::{Node, Type},
    SubType,
};

/// # References
/// * [NVDIMM Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#nvdimm-device-path)
#[derive(Debug)]
#[repr(C)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
    nfit_device_handle: u32,
}

impl Node for Protocol {}
