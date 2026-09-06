#![no_std]

extern crate alloc;

use {alloc::vec::Vec, core::ops::Range};

pub struct Region(Range<usize>);

impl From<Range<usize>> for Region {
    fn from(range: Range<usize>) -> Self {
        Self(range)
    }
}

impl From<&Range<usize>> for Region {
    fn from(range: &Range<usize>) -> Self {
        Self(range.clone())
    }
}

pub struct Regions(Vec<Region>);
