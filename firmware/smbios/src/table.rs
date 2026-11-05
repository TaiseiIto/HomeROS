pub mod processor_information;
pub mod system;

use {
    crate::{Header, Type, header},
    alloc::{string::String, vec::Vec},
    core::fmt::Debug,
};

pub trait Table: Debug {}

pub struct List<'a>(header::List<'a>);

impl<'a> From<header::List<'a>> for List<'a> {
    fn from(header_list: header::List<'a>) -> Self {
        Self(header_list)
    }
}

impl<'a> Iterator for List<'a> {
    type Item = Result<&'a dyn Table, Type>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(Header::table)
    }
}
