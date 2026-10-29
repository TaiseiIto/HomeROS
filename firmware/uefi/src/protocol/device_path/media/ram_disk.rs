use {
    super::{
        super::{Node, Type},
        SubType,
    },
    crate::Guid,
};

/// # References
/// * [RAM Disk](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#ram-disk)
#[derive(Debug)]
#[repr(C, packed)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
    starting_address: u64,
    ending_address: u64,
    disk_type: Guid,
    instakce: u16,
}

impl Node for Protocol {}
