use super::{
    super::{Node, Type},
    SubType,
};

/// # References
/// * [UFS (Universal Flash Storage) device messaging devicepath node](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#ufs-universal-flash-storage-device-messaging-devicepath-node)
#[derive(Debug, packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
    target_id: u8,
    lun: u8,
}

impl Node for Protocol {}
