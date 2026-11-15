use {
    super::{
        super::{Node, Type},
        SubType,
    },
    crate::Char16,
    alloc::string::String,
    core::{
        fmt::{self, Debug, Formatter},
        marker::PhantomData,
        mem::{offset_of, size_of},
        slice::from_raw_parts,
    },
};

/// # References
/// * [File Path Media Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#file-path-media-device-path)
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    typ: u8,
    sub_type: u8,
    length: u16,
    name: PhantomData<Char16>,
}

impl Protocol {
    fn get_type(&self) -> Type {
        self.typ().into()
    }

    fn get_sub_type(&self) -> SubType {
        self.sub_type().into()
    }

    fn name_bytes(&self) -> &[Char16] {
        unsafe {
            from_raw_parts(
                &self.name as *const PhantomData<Char16> as *const Char16,
                self.name_length(),
            )
        }
    }

    fn name_string(&self) -> String {
        char::decode_utf16(self.name_bytes().iter().cloned())
            .filter_map(Result::ok)
            .collect()
    }

    fn name_length(&self) -> usize {
        self.name_size() / size_of::<Char16>()
    }

    fn name_size(&self) -> usize {
        self.length as usize - offset_of!(Self, name)
    }
}

impl Debug for Protocol {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Protocol")
            .field("typ", &self.get_type())
            .field("sub_type", &self.get_sub_type())
            .field("length", &self.length())
            .field("name", &self.name_string())
            .finish()
    }
}

impl Node for Protocol {}
