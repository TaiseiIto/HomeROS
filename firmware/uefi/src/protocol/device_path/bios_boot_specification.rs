use {
    super::{Node, Type},
    core::{
        fmt::{Debug, Formatter, Result},
        marker::PhantomData,
        mem::offset_of,
        slice::from_raw_parts,
    },
};

/// # References
/// * [BIOS Boot Specification Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#bios-boot-specification-device-path)
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
    device_type: u16,
    status_flag: u16,
    description_string: PhantomData<u8>,
}

impl Protocol {
    fn description_string_str(&self) -> &str {
        str::from_utf8(self.description_string_bytes()).unwrap()
    }

    fn description_string_bytes(&self) -> &[u8] {
        unsafe {
            from_raw_parts(
                &self.description_string as *const PhantomData<u8> as *const u8,
                self.description_string_size(),
            )
        }
    }

    fn description_string_size(&self) -> usize {
        self.length() as usize - offset_of!(Self, description_string)
    }
}

impl Debug for Protocol {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Protocol")
            .field("ty", &self.ty())
            .field("sub_type", &self.sub_type())
            .field("length", &self.length())
            .field("device_type", &self.device_type())
            .field("status_flag", &self.status_flag())
            .field("description_string", &self.description_string_str())
            .finish()
    }
}

impl Node for Protocol {}

/// # References
/// * [BIOS Boot Specification Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#bios-boot-specification-device-path)
#[derive(Debug)]
#[repr(u8)]
enum SubType {
    Version101 = 0x01,
}
