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
/// * [ACPI _ADR Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#acpi-adr-device-path)
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    typ: u8,
    sub_type: u8,
    length: u16,
    adr: u32,
    additional: PhantomData<u8>,
}

impl Protocol {
    fn additional_bytes(&self) -> &[u8] {
        unsafe {
            from_raw_parts(
                &self.additional as *const PhantomData<u8> as *const u8,
                self.additional_size(),
            )
        }
    }

    fn additional_size(&self) -> usize {
        self.length as usize - offset_of!(Self, additional)
    }

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
            .field("adr", &self.adr())
            .field("additional", &self.additional_bytes())
            .finish()
    }
}

impl Node for Protocol {}
