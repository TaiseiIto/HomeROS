use {
    crate::Header,
    alloc::{string::String, vec::Vec},
    core::fmt::{Debug, Formatter, Result},
};

#[repr(transparent)]
pub struct Table(Header);

impl Table {
    fn filter(&self) -> Option<&Self> {
        self.0.filter().map(|_| self)
    }

    fn next_table(&self) -> &Self {
        unsafe { &*(self.0.next_header() as *const Header as *const Self) }
    }

    fn strings(&self) -> Vec<String> {
        self.0.strings().collect()
    }
}

impl Debug for Table {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Table")
            .field("header", &self.0)
            .field("strings", &self.strings())
            .finish()
    }
}

pub struct List<'a>(&'a Table);

impl<'a> From<&'a Table> for List<'a> {
    fn from(table: &'a Table) -> Self {
        Self(table)
    }
}

impl<'a> Iterator for List<'a> {
    type Item = &'a Table;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.filter().inspect(|_| self.0 = self.0.next_table())
    }
}
