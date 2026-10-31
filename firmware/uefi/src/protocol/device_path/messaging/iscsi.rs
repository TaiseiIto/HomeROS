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
/// * [iSCSI Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#iscsi-device-path)
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
    protocol: u16,
    options: u16,
    number: u64,
    target_portal_group_tag: u16,
    name: PhantomData<u8>,
}

impl Protocol {
    fn name_str(&self) -> &str {
        str::from_utf8(self.name_bytes()).unwrap()
    }

    fn name_bytes(&self) -> &[u8] {
        unsafe {
            from_raw_parts(
                &self.name as *const PhantomData<u8> as *const u8,
                self.name_size(),
            )
        }
    }

    fn name_size(&self) -> usize {
        self.length() as usize - offset_of!(Self, name)
    }
}

impl Debug for Protocol {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Protocol")
            .field("ty", &self.ty())
            .field("sub_type", &self.sub_type())
            .field("length", &self.length())
            .field("protocol", &self.protocol())
            .field("options", &self.options())
            .field("number", &self.number())
            .field("target_portal_group_tag", &self.target_portal_group_tag())
            .field("name", &self.name_str())
            .finish()
    }
}

impl Node for Protocol {}
