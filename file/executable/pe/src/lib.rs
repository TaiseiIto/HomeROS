//! # Portable Executable
//! ## References
//! * [A dive into the PE file format - PE file structure](https://0xrick.github.io/win-internals/pe2/)

#![no_std]

mod dos;

use core::fmt::{Debug, Formatter, Result};

pub struct File<'a>(&'a [u8]);

impl<'a> File<'a> {
    fn dos_header(&'a self) -> &'a dos::Header {
        let dos_header: *const dos::Header = self.0.as_ptr() as *const dos::Header;
        unsafe { &*dos_header }
    }
}

impl<'a> Debug for File<'a> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("File")
            .field("dos_header", self.dos_header())
            .finish()
    }
}

impl<'a> From<&'a [u8]> for File<'a> {
    fn from(file: &'a [u8]) -> Self {
        Self(file)
    }
}
