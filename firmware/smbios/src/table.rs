/// # References
/// * [SMBIOS Specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.10.0.pdf) 6.1.2 Structure header format
#[derive(Debug, packed::Accessor)]
#[repr(C, packed)]
pub struct Header {
    typ: u8,
    length: u8,
    handle: u16,
}

impl Header {
    fn filter(&self) -> Option<&Self> {
        (!self.is_end_of_table()).then_some(self)
    }

    /// # References
    /// * [SMBIOS Specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.10.0.pdf) 7.49 End-of-Table (Type 127)
    fn is_end_of_table(&self) -> bool {
        self.typ() == 127
    }

    fn next_header(&self) -> &Self {
        unsafe { &*(self as *const Self).byte_add(self.size()) }
    }

    fn size(&self) -> usize {
        self.length() as usize
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
        self.0.filter().inspect(|item| {
            self.0 = item.next_header();
        })
    }
}
