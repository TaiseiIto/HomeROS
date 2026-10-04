use {
    crate::header::Header,
    core::fmt::{Debug, Formatter, Result},
    memory::Regions,
};

/// # References
/// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 5.3.2 Format
#[derive(Clone, endian::Big)]
#[repr(C)]
pub struct Entry {
    address: u64,
    size: u64,
}

impl Entry {
    fn check(self) -> Option<Self> {
        (self.read_address() != 0 || self.read_size() != 0).then_some(self)
    }
}

impl Debug for Entry {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Entry")
            .field("address", &self.read_address())
            .field("size", &self.read_size())
            .finish()
    }
}

impl From<Entry> for Regions<u128> {
    fn from(entry: Entry) -> Self {
        let address: u128 = entry.address as u128;
        let size: u128 = entry.size as u128;
        let start: u128 = address;
        let end: u128 = start + size;
        (start..end).try_into().unwrap()
    }
}

pub struct EntryIterator<'a> {
    entry: &'a Entry,
}

impl<'a> From<&'a Header> for EntryIterator<'a> {
    fn from(header: &'a Header) -> Self {
        Self {
            entry: header.reserved_memory_entry(),
        }
    }
}

impl Iterator for EntryIterator<'_> {
    type Item = Entry;

    fn next(&mut self) -> Option<Self::Item> {
        let entry: Option<Self::Item> = self.entry.clone().check();
        self.entry = unsafe { &*(self.entry as *const Entry).add(1) };
        entry
    }
}
