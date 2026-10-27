use {
    super::{
        super::{Node, Type},
        SubType,
    },
    crate::Char16,
    core::{
        marker::PhantomData,
        mem::{offset_of, size_of},
        slice::from_raw_parts,
    },
};

/// # References
/// * [File Path Media Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#file-path-media-device-path)
#[derive(Debug)]
#[repr(C)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
    name: PhantomData<Char16>,
}

impl Protocol {
    fn name(&self) -> &[Char16] {
        unsafe {
            from_raw_parts(
                &self.name as *const PhantomData<Char16> as *const Char16,
                self.name_length(),
            )
        }
    }

    fn name_length(&self) -> usize {
        self.name_size() / size_of::<Char16>()
    }

    fn name_size(&self) -> usize {
        self.length as usize - offset_of!(Self, name)
    }
}

impl Node for Protocol {}
