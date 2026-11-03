use {
    super::{
        super::{Node, Type},
        SubType,
    },
    alloc::vec::Vec,
    core::{
        fmt::{Debug, Formatter, Result},
        marker::PhantomData,
        mem::offset_of,
        slice::from_raw_parts,
    },
};

/// # References
/// * [ACPI Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#acpi-device-path)
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    typ: u8,
    sub_type: u8,
    length: u16,
    hid: u32,
    uid: u32,
    cid: u32,
    hidstr: PhantomData<u8>,
}

impl Protocol {
    fn get_type(&self) -> Type {
        self.typ().into()
    }

    fn get_sub_type(&self) -> SubType {
        self.sub_type().into()
    }

    fn hid_str(&self) -> &str {
        self.strings()[0]
    }

    fn uid_str(&self) -> &str {
        self.strings()[1]
    }

    fn cid_str(&self) -> &str {
        self.strings()[2]
    }

    fn strings(&self) -> [&str; 3] {
        let strings: Vec<&str> = self
            .strings_bytes()
            .split(|byte| *byte == 0x00)
            .filter_map(|string| str::from_utf8(string).ok())
            .collect();
        strings.try_into().unwrap()
    }

    fn strings_bytes(&self) -> &[u8] {
        unsafe {
            from_raw_parts(
                &self.hidstr as *const PhantomData<u8> as *const u8,
                self.strings_size(),
            )
        }
    }

    fn strings_size(&self) -> usize {
        self.length() as usize - offset_of!(Self, hidstr)
    }
}

impl Debug for Protocol {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Protocol")
            .field("typ", &self.get_type())
            .field("sub_type", &self.get_sub_type())
            .field("length", &self.length())
            .field("hid", &self.hid())
            .field("uid", &self.uid())
            .field("cid", &self.cid())
            .field("hidstr", &self.hid_str())
            .field("uidstr", &self.uid_str())
            .field("cidstr", &self.cid_str())
            .finish()
    }
}

impl Node for Protocol {}
