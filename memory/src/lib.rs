#![no_std]

extern crate alloc;

use {alloc::vec::Vec, core::ops::Range};

#[derive(Clone, Debug)]
pub struct Region(Range<usize>);

impl Region {
    fn try_merge(&self, other: &Self) -> Option<Self> {
        let Self(Range {
            start: self_start,
            end: self_end,
        }) = self;
        let Self(Range {
            start: other_start,
            end: other_end,
        }) = other;
        if self_start < other_start {
            if self_end < other_start {
                // self_start < self_end < other_start < other_end
                None
            } else {
                // self_start < other_start <= self_end
                if self_end < other_end {
                    // self_start < other_start <= self_end < other_end
                    (self_start..other_end).try_into().ok()
                } else {
                    // self_start < other_start < other_end <= self_end
                    Some(self.clone())
                }
            }
        } else {
            // other_start <= self_start
            if other_end < self_start {
                // other_start < other_end < self_start < self_end
                None
            } else {
                // other_start <= self_start <= other_end
                if other_end < self_end {
                    // other_start <= self_start <= other_end < self_end
                    (other_start..self_end).try_into().ok()
                } else {
                    // other_start <= self_start < self_end <= other_end
                    Some(other.clone())
                }
            }
        }
    }
}

impl TryFrom<Range<usize>> for Region {
    type Error = ();

    fn try_from(range: Range<usize>) -> Result<Self, Self::Error> {
        if range.is_empty() {
            Err(())
        } else {
            Ok(Self(range))
        }
    }
}

impl TryFrom<Range<&usize>> for Region {
    type Error = ();

    fn try_from(range: Range<&usize>) -> Result<Self, Self::Error> {
        let Range { start, end } = range;
        (*start..*end).try_into()
    }
}

impl TryFrom<&Range<usize>> for Region {
    type Error = ();

    fn try_from(range: &Range<usize>) -> Result<Self, Self::Error> {
        range.clone().try_into()
    }
}

impl TryFrom<&Range<&usize>> for Region {
    type Error = ();

    fn try_from(range: &Range<&usize>) -> Result<Self, Self::Error> {
        range.clone().try_into()
    }
}

pub struct Regions(Vec<Region>);
