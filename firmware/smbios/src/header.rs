use {
    crate::Strings,
    alloc::{string::String, vec::Vec},
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [SMBIOS Specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.10.0.pdf) 6.1.2 Structure header format
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Header {
    typ: u8,
    length: u8,
    handle: u16,
}

impl Header {
    pub fn filter(&self) -> Option<&Self> {
        (!self.is_end_of_table()).then_some(self)
    }

    pub fn next_header(&self) -> &Self {
        unsafe { &*(self as *const Self).byte_add(self.size() + self.strings().size()) }
    }

    pub fn strings<'a>(&'a self) -> Strings<'a> {
        unsafe { &*(self as *const Self as *const u8).byte_add(self.size()) }.into()
    }

    /// # References
    /// * [SMBIOS Specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.10.0.pdf) 7.49 End-of-Table (Type 127)
    fn is_end_of_table(&self) -> bool {
        self.typ() == 127
    }

    fn size(&self) -> usize {
        self.length() as usize
    }
}

impl Debug for Header {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        let strings: Vec<String> = self.strings().collect();
        formatter
            .debug_struct("Header")
            .field("typ", &self.typ())
            .field("length", &self.length())
            .field("handle", &self.handle())
            .field("strings", &strings)
            .finish()
    }
}

pub struct List<'a>(&'a Header);

impl<'a> From<&'a Header> for List<'a> {
    fn from(header: &'a Header) -> Self {
        Self(header)
    }
}

impl<'a> Iterator for List<'a> {
    type Item = &'a Header;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.filter().inspect(|_| self.0 = self.0.next_header())
    }
}
