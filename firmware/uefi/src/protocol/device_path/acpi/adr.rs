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
#[repr(C)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
    adr: u32,
    additional: PhantomData<u8>,
}

impl Protocol {
    fn additional(&self) -> &[u8] {
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
}

impl Debug for Protocol {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Protocol")
            .field("ty", &self.ty)
            .field("sub_type", &self.sub_type)
            .field("length", &self.length)
            .field("adr", &self.adr)
            .field("additional", &self.additional())
            .finish()
    }
}

impl Node for Protocol {}
