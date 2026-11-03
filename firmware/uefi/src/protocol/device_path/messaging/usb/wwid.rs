use {
    super::super::{
        super::{Node, Type},
        SubType,
    },
    core::{
        fmt::{Debug, Formatter, Result},
        marker::PhantomData,
        mem::offset_of,
        slice::from_raw_parts,
    },
};

/// # References
/// * [USB Device Paths (WWID)](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#usb-device-paths-wwid)
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    typ: u8,
    sub_type: u8,
    length: u16,
    interface_number: u16,
    device_vendor_id: u16,
    device_product_id: u16,
    serial_number: PhantomData<u8>,
}

impl Protocol {
    fn get_type(&self) -> Type {
        self.typ().into()
    }

    fn get_sub_type(&self) -> SubType {
        self.sub_type().into()
    }

    fn serial_number_bytes(&self) -> &[u8] {
        unsafe {
            from_raw_parts(
                &self.serial_number as *const PhantomData<u8> as *const u8,
                self.serial_number_size(),
            )
        }
    }

    fn serial_number_size(&self) -> usize {
        self.length() as usize - offset_of!(Self, serial_number)
    }
}

impl Debug for Protocol {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Protocol")
            .field("typ", &self.get_type())
            .field("sub_type", &self.get_sub_type())
            .field("length", &self.length())
            .field("interface_number", &self.interface_number())
            .field("device_vendor_id", &self.device_vendor_id())
            .field("device_product_id", &self.device_product_id())
            .field("serial_number", &self.serial_number_bytes())
            .finish()
    }
}

impl Node for Protocol {}
