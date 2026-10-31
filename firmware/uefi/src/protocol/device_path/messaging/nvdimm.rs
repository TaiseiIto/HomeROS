use {
    super::{
        super::{Node, Type},
        SubType,
    },
    crate::Guid,
};

/// # References
/// * [NVDIMM Namespace path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#nvdimm-namespace-path)
#[derive(Debug, packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
    uuid: Guid,
}

impl Node for Protocol {}
