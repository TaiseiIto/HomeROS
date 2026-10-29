use {
    super::{
        super::{Node, Type},
        SubType,
    },
    crate::Guid,
    core::{
        fmt::{Debug, Formatter, Result},
        marker::PhantomData,
        mem::offset_of,
        ptr::addr_of,
        slice::from_raw_parts,
    },
};

/// # References
/// * [Vendor-Devined Media Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#cd-rom-media-device-path)
#[repr(C, packed)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
    guid: Guid,
    data: PhantomData<u8>,
}

impl Protocol {
    fn length(&self) -> u16 {
        unsafe { addr_of!(self.length).read_unaligned() }
    }

    fn guid(&self) -> Guid {
        unsafe { addr_of!(self.guid).read_unaligned() }
    }

    fn data(&self) -> &[u8] {
        unsafe {
            from_raw_parts(
                &self.data as *const PhantomData<u8> as *const u8,
                self.data_length(),
            )
        }
    }

    fn data_length(&self) -> usize {
        self.length as usize - offset_of!(Self, data)
    }
}

impl Debug for Protocol {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Protocol")
            .field("ty", &self.ty)
            .field("sub_type", &self.sub_type)
            .field("length", &self.length())
            .field("guid", &self.guid())
            .field("data", &self.data())
            .finish()
    }
}

impl Node for Protocol {}
