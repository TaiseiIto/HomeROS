use {
    super::{
        super::{Node, Type},
        SubType,
    },
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [UFS (Universal Flash Storage) device messaging devicepath node](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#ufs-universal-flash-storage-device-messaging-devicepath-node)
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    typ: u8,
    sub_type: u8,
    length: u16,
    target_id: u8,
    lun: u8,
}

impl Protocol {
    fn get_type(&self) -> Type {
        self.typ().into()
    }

    fn get_sub_type(&self) -> SubType {
        self.sub_type().into()
    }
}

impl Debug for Protocol {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Protocol")
            .field("typ", &self.get_type())
            .field("sub_type", &self.get_sub_type())
            .field("length", &self.length())
            .field("target_id", &self.target_id())
            .field("lun", &self.lun())
            .finish()
    }
}

impl Node for Protocol {}
