use {
    super::super::{
        super::{Node, Type},
        SubType,
    },
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [Device Logical Unit](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#device-logical-unit)
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    typ: u8,
    sub_type: u8,
    length: u16,
    vendor_id: u16,
    product_id: u16,
    device_class: u8,
    device_subclass: u8,
    device_protocol: u8,
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
            .field("vendor_id", &self.vendor_id())
            .field("product_id", &self.product_id())
            .field("device_class", &self.device_class())
            .field("device_subclass", &self.device_subclass())
            .field("device_protocol", &self.device_protocol())
            .finish()
    }
}

impl Node for Protocol {}
