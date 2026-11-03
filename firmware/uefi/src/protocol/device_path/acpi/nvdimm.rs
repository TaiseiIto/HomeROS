use super::super::Node;

/// # References
/// * [NVDIMM Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#nvdimm-device-path)
#[derive(Debug, packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    ty: u8,
    sub_type: u8,
    length: u16,
    nfit_device_handle: u32,
}

impl Node for Protocol {}
