mod uart_flow_control;

use {
    super::{
        super::{Node, Type},
        SubType,
    },
    crate::Guid,
    core::fmt::{self, Debug, Formatter},
};

/// # References
/// * [Vendor-Defined Messaging Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#vendor-defined-messaging-device-path)
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
    guid: Guid,
}

impl Protocol {
    fn read(&self) -> Result<&dyn Node, Guid> {
        match self.guid() {
            uart_flow_control::Protocol::GUID => {
                Ok(unsafe { &*(self as *const Self as *const uart_flow_control::Protocol) })
            }
            guid => Err(guid),
        }
    }
}

impl Debug for Protocol {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        self.read().fmt(formatter)
    }
}

impl Node for Protocol {}
