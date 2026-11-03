use {super::super::Node, crate::Guid};

/// # References
/// * [RAM Disk](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#ram-disk)
#[derive(Debug, packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    ty: u8,
    sub_type: u8,
    length: u16,
    starting_address: u64,
    ending_address: u64,
    disk_type: Guid,
    instakce: u16,
}

impl Node for Protocol {}
