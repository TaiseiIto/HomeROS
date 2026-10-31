use {
    super::{
        super::{Node, Type},
        SubType,
    },
    crate::Guid,
    core::{
        fmt::{Debug, Formatter, Result},
        marker::PhantomData,
        mem::{offset_of, size_of},
        slice::from_raw_parts,
    },
};

/// # References
/// * [REST Service Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#rest-service-device-path)
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
    rest_service: u8,
    access_mode: u8,
    vendor_specific: PhantomData<u8>,
}

impl Protocol {
    fn vendor_data(&self) -> Option<&[u8]> {
        self.vendor_specific_bytes()
            .split_at_checked(size_of::<Guid>())
            .map(|(_, data)| data)
    }

    fn vendor_guid(&self) -> Option<Guid> {
        self.vendor_specific_bytes()
            .split_at_checked(size_of::<Guid>())
            .map(|(guid, _)| unsafe { (guid.as_ptr() as *const Guid).read_unaligned() })
    }

    fn vendor_specific_bytes(&self) -> &[u8] {
        unsafe {
            from_raw_parts(
                &self.vendor_specific as *const PhantomData<u8> as *const u8,
                self.vendor_specific_size(),
            )
        }
    }

    fn vendor_specific_size(&self) -> usize {
        self.length() as usize - offset_of!(Self, vendor_specific)
    }
}

impl Debug for Protocol {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Protocol")
            .field("ty", &self.ty())
            .field("sub_type", &self.sub_type())
            .field("length", &self.length())
            .field("rest_service", &self.rest_service())
            .field("access_mode", &self.access_mode())
            .field("vendor_guid", &self.vendor_guid())
            .field("vendor_data", &self.vendor_data())
            .finish()
    }
}

impl Node for Protocol {}
