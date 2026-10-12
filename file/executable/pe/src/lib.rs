//! # Portable Executable
//! ## References
//! * [A dive into the PE file format - PE file structure](https://0xrick.github.io/win-internals/pe2/)

#![no_std]

extern crate alloc;

mod data;
mod dos;
mod file;
mod nt;
mod optional;
mod section;

use core::fmt::{Debug, Formatter, Result};

pub struct File<'a>(&'a [u8]);

impl<'a> File<'a> {
    fn dos_header(&'a self) -> &'a dos::Header {
        let dos_header: *const dos::Header = self.0.as_ptr() as *const dos::Header;
        unsafe { &*dos_header }
    }

    fn nt_header(&'a self) -> &'a nt::Header {
        let nt_header: *const u8 =
            self.0.get(self.dos_header().nt_header_offset()).unwrap() as *const u8;
        let nt_header: *const nt::Header = nt_header as *const nt::Header;
        unsafe { &*nt_header }
    }
}

impl<'a> Debug for File<'a> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("File")
            .field("dos_header", self.dos_header())
            .field("nt_header", self.nt_header())
            .field("section_table", &self.nt_header().section_table())
            .finish()
    }
}

impl<'a> From<&'a [u8]> for File<'a> {
    fn from(file: &'a [u8]) -> Self {
        Self(file)
    }
}
