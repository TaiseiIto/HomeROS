use {
    crate::service::boot::memory::Type,
    core::fmt::{Debug, Formatter, Result},
};

pub struct Table;

impl Table {
    fn iter(&self) -> EntryIterator<'_> {
        EntryIterator(unsafe { &*(self as *const Self as *const Entry) })
    }
}

impl Debug for Table {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter.debug_list().entries(self.iter()).finish()
    }
}

struct EntryIterator<'a>(&'a Entry);

impl<'a> Iterator for EntryIterator<'a> {
    type Item = &'a Entry;

    fn next(&mut self) -> Option<Self::Item> {
        let item: Self::Item = self.0;
        self.0 = unsafe { &*(self.0 as *const Entry).add(1) };
        item.filter()
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Entry {
    ty: Type,
    pages: u32,
}

impl Entry {
    const END: Self = Self {
        ty: Type::Max,
        pages: 0,
    };

    fn filter(&self) -> Option<&Self> {
        (*self != Self::END).then_some(self)
    }
}
