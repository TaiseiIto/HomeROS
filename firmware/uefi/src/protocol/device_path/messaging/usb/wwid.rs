use {
    super::super::{
        super::{Node, Type},
        SubType,
    },
    core::{
        fmt::{Debug, Formatter, Result},
        marker::PhantomData,
        mem::offset_of,
        ptr::addr_of,
        slice::from_raw_parts,
    },
};

/// # References
/// * [USB Device Paths (WWID)](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#usb-device-paths-wwid)
#[repr(C, packed)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
    interface_number: u16,
    device_vendor_id: u16,
    device_product_id: u16,
    serial_number: PhantomData<u8>,
}

impl Protocol {
    fn length(&self) -> u16 {
        unsafe { addr_of!(self.length).read_unaligned() }
    }

    fn interface_number(&self) -> u16 {
        unsafe { addr_of!(self.interface_number).read_unaligned() }
    }

    fn device_vendor_id(&self) -> u16 {
        unsafe { addr_of!(self.device_vendor_id).read_unaligned() }
    }

    fn device_product_id(&self) -> u16 {
        unsafe { addr_of!(self.device_product_id).read_unaligned() }
    }

    fn serial_number(&self) -> &[u8] {
        unsafe {
            from_raw_parts(
                &self.serial_number as *const PhantomData<u8> as *const u8,
                self.serial_number_size(),
            )
        }
    }

    fn serial_number_size(&self) -> usize {
        self.length as usize - offset_of!(Self, serial_number)
    }
}

impl Debug for Protocol {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Protocol")
            .field("ty", &self.ty)
            .field("sub_type", &self.sub_type)
            .field("length", &self.length())
            .field("interface_number", &self.interface_number())
            .field("device_vendor_id", &self.device_vendor_id())
            .field("device_product_id", &self.device_product_id())
            .field("serial_number", &self.serial_number())
            .finish()
    }
}

impl Node for Protocol {}
