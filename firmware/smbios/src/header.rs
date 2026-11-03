use {
    crate::{Strings, Table, table::system},
    alloc::{string::String, vec::Vec},
    core::fmt::{self, Debug, Formatter},
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
        (!matches!(self.get_type(), Type::EndOfTable)).then_some(self)
    }

    pub fn next_header(&self) -> &Self {
        unsafe { &*(self as *const Self).byte_add(self.size() + self.strings().size()) }
    }

    pub fn strings<'a>(&'a self) -> Strings<'a> {
        unsafe { &*(self as *const Self as *const u8).byte_add(self.size()) }.into()
    }

    /// # TODO
    /// * Implement all table types.
    pub fn table(&self) -> Result<&dyn Table, Type> {
        match self.get_type() {
            Type::SystemInformation => {
                Ok(unsafe { &*(self as *const Self as *const system::information::Table) })
            }
            Type::SystemEnclosure => {
                Ok(unsafe { &*(self as *const Self as *const system::enclosure::Table) })
            }
            typ => Err(typ),
        }
    }

    fn size(&self) -> usize {
        self.length() as usize
    }

    fn get_type(&self) -> Type {
        self.typ().into()
    }
}

impl Debug for Header {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        let strings: Vec<String> = self.strings().collect();
        formatter
            .debug_struct("Header")
            .field("typ", &self.get_type())
            .field("length", &self.length())
            .field("handle", &self.handle())
            .field("strings", &strings)
            .finish()
    }
}

#[derive(Debug)]
pub enum Type {
    SystemInformation,
    SystemEnclosure,
    /// # References
    /// * [SMBIOS Specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.10.0.pdf) 7.49 End-of-Table (Type 127)
    EndOfTable,
    Unknown(u8),
}

impl From<u8> for Type {
    fn from(typ: u8) -> Self {
        match typ {
            1 => Self::SystemInformation,
            3 => Self::SystemEnclosure,
            127 => Self::EndOfTable,
            typ => Self::Unknown(typ),
        }
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
