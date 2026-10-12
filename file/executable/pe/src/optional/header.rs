use {
    super::Header,
    core::{
        fmt::{Debug, DebugStruct, Formatter, Result},
        mem::{offset_of, size_of_val},
    },
};

pub struct WithSize<'a> {
    header: &'a Header,
    size: usize,
}

impl<'a> WithSize<'a> {
    pub fn new(header: &'a Header, size: usize) -> Self {
        Self { header, size }
    }

    fn magic(&self) -> Option<u16> {
        (offset_of!(Header, magic) + size_of_val(&self.header.magic) <= self.size)
            .then_some(self.header.magic)
    }
}

impl Debug for WithSize<'_> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        let mut debug_struct: DebugStruct = formatter.debug_struct("Header");
        if let Some(magic) = self.magic() {
            debug_struct.field("magic", &magic);
        }
        debug_struct.finish()
    }
}
