#![no_std]

extern crate alloc;

use {
    alloc::{vec, vec::Vec},
    core::ops::Range,
};

#[derive(Clone, Debug, Eq, PartialEq)]
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

impl TryFrom<Range<usize>> for Regions {
    type Error = ();

    fn try_from(range: Range<usize>) -> Result<Self, Self::Error> {
        range.try_into().map(|range| Self(vec![range]))
    }
}

impl TryFrom<Range<&usize>> for Regions {
    type Error = ();

    fn try_from(range: Range<&usize>) -> Result<Self, Self::Error> {
        range.try_into().map(|range| Self(vec![range]))
    }
}

impl TryFrom<&Range<usize>> for Regions {
    type Error = ();

    fn try_from(range: &Range<usize>) -> Result<Self, Self::Error> {
        range.try_into().map(|range| Self(vec![range]))
    }
}

impl TryFrom<&Range<&usize>> for Regions {
    type Error = ();

    fn try_from(range: &Range<&usize>) -> Result<Self, Self::Error> {
        range.try_into().map(|range| Self(vec![range]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn region_try_merge() {
        let a: Region = (0..1).try_into().unwrap();
        let b: Region = (2..3).try_into().unwrap();
        assert_eq!(a.try_merge(&b), None);
        assert_eq!(b.try_merge(&a), None);
        let a: Region = (0..1).try_into().unwrap();
        let b: Region = (1..2).try_into().unwrap();
        assert_eq!(a.try_merge(&b), (0..2).try_into().ok());
        assert_eq!(b.try_merge(&a), (0..2).try_into().ok());
        let a: Region = (0..2).try_into().unwrap();
        let b: Region = (1..3).try_into().unwrap();
        assert_eq!(a.try_merge(&b), (0..3).try_into().ok());
        assert_eq!(b.try_merge(&a), (0..3).try_into().ok());
        let a: Region = (0..2).try_into().unwrap();
        let b: Region = (0..1).try_into().unwrap();
        assert_eq!(a.try_merge(&b), (0..2).try_into().ok());
        assert_eq!(b.try_merge(&a), (0..2).try_into().ok());
        let a: Region = (0..2).try_into().unwrap();
        let b: Region = (1..2).try_into().unwrap();
        assert_eq!(a.try_merge(&b), (0..2).try_into().ok());
        assert_eq!(b.try_merge(&a), (0..2).try_into().ok());
        let a: Region = (0..3).try_into().unwrap();
        let b: Region = (1..2).try_into().unwrap();
        assert_eq!(a.try_merge(&b), (0..3).try_into().ok());
        assert_eq!(b.try_merge(&a), (0..3).try_into().ok());
    }
}
