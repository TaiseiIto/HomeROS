pub mod processor_information;
pub mod system;

use {
    crate::{Header, header},
    alloc::{string::String, vec::Vec},
    core::fmt::Debug,
};

pub trait Table: Debug {
    fn get_header(&self) -> &Header {
        unsafe { &*(self as *const Self as *const Header) }
    }

    fn strings(&self) -> Vec<String> {
        self.get_header().strings().collect()
    }
}

pub struct List<'a>(header::List<'a>);

impl<'a> From<header::List<'a>> for List<'a> {
    fn from(header_list: header::List<'a>) -> Self {
        Self(header_list)
    }
}

impl<'a> Iterator for List<'a> {
    type Item = &'a dyn Table;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().and_then(Header::table)
    }
}
