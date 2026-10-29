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
        ptr::addr_of,
        slice::from_raw_parts,
    },
};

/// # References
/// * [ACPI Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#acpi-device-path)
#[repr(C, packed)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
    hid: u32,
    uid: u32,
    cid: u32,
    hidstr: PhantomData<u8>,
}

impl Protocol {
    fn length(&self) -> u16 {
        unsafe { addr_of!(self.length).read_unaligned() }
    }

    fn hid(&self) -> u32 {
        unsafe { addr_of!(self.hid).read_unaligned() }
    }

    fn uid(&self) -> u32 {
        unsafe { addr_of!(self.uid).read_unaligned() }
    }

    fn cid(&self) -> u32 {
        unsafe { addr_of!(self.cid).read_unaligned() }
    }

    fn hidstr(&self) -> &str {
        self.strings()[0]
    }

    fn uidstr(&self) -> &str {
        self.strings()[1]
    }

    fn cidstr(&self) -> &str {
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
        self.length as usize - offset_of!(Self, hidstr)
    }
}

impl Debug for Protocol {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Protocol")
            .field("ty", &self.ty)
            .field("sub_type", &self.sub_type)
            .field("length", &self.length())
            .field("hid", &self.hid())
            .field("uid", &self.uid())
            .field("cid", &self.cid())
            .field("hidstr", &self.hidstr())
            .field("uidstr", &self.uidstr())
            .field("cidstr", &self.cidstr())
            .finish()
    }
}

impl Node for Protocol {}
