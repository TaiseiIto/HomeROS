use {
    super::{
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
/// * [Uniform Resource Identifiers (URI) Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#uniform-resource-identifiers-uri-device-path)
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
    data: PhantomData<u8>,
}

impl Protocol {
    fn data_bytes(&self) -> &[u8] {
        unsafe {
            from_raw_parts(
                &self.data as *const PhantomData<u8> as *const u8,
                self.data_size(),
            )
        }
    }

    fn data_size(&self) -> usize {
        self.length() as usize - offset_of!(Self, data)
    }
}

impl Debug for Protocol {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Protocol")
            .field("ty", &self.ty())
            .field("sub_type", &self.sub_type())
            .field("length", &self.length())
            .field("data", &self.data_bytes())
            .finish()
    }
}

impl Node for Protocol {}
