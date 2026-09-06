#![no_std]

extern crate alloc;

use {
    alloc::{vec, vec::Vec},
    core::{
        fmt::{self, Debug, Formatter},
        iter::Sum,
        ops::{Add, Range, Sub},
    },
};

#[derive(Clone, Eq, PartialEq)]
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
                    (*self_start..*other_end).try_into().ok()
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
                    (*other_start..*self_end).try_into().ok()
                } else {
                    // other_start <= self_start < self_end <= other_end
                    Some(other.clone())
                }
            }
        }
    }
}

impl Add for Region {
    type Output = Regions;

    fn add(self, other: Self) -> Self::Output {
        let left: Regions = self.into();
        let right: Regions = other.into();
        left + right
    }
}

impl Debug for Region {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl Sub for Region {
    type Output = Regions;

    fn sub(self, other: Self) -> Self::Output {
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
                (self_start..self_end).try_into().unwrap()
            } else {
                // self_start < other_start <= self_end
                if self_end < other_end {
                    // self_start < other_start <= self_end < other_end
                    (self_start..other_start).try_into().unwrap()
                } else {
                    // self_start < other_start < other_end <= self_end
                    [self_start..other_start, other_end..self_end]
                        .as_slice()
                        .into()
                }
            }
        } else {
            // other_start <= self_start
            if other_end < self_start {
                // other_start < other_end < self_start < self_end
                (self_start..self_end).try_into().unwrap()
            } else {
                // other_start <= self_start <= other_end
                if other_end < self_end {
                    // other_start <= self_start <= other_end < self_end
                    (other_end..self_end).try_into().unwrap()
                } else {
                    // other_start <= self_start < self_end <= other_end
                    Self::Output::default()
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

#[derive(Clone, Default, Eq, PartialEq)]
pub struct Regions(Vec<Region>);

impl Regions {
    fn deduplicate(&mut self) {
        if let Some(head) = self.0.pop() {
            self.deduplicate();
            let mut head_is_merged: bool = false;
            for region in self.0.iter_mut() {
                if let Some(merged) = head.try_merge(region) {
                    *region = merged;
                    head_is_merged = true;
                    break;
                }
            }
            if head_is_merged {
                self.deduplicate();
            } else {
                self.0.push(head);
            }
        }
    }

    fn normalize(&mut self) {
        self.deduplicate();
        self.sort();
    }

    fn sort(&mut self) {
        self.0.sort_by_key(|region| region.0.start);
    }
}

impl Add for Regions {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut output: Self = Self(self.0.into_iter().chain(other.0).collect());
        output.normalize();
        output
    }
}

impl Debug for Regions {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.debug_list().entries(self.0.iter()).finish()
    }
}

impl From<Region> for Regions {
    fn from(region: Region) -> Self {
        Self(vec![region])
    }
}

impl Sub for Regions {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self.0
            .into_iter()
            .map(|self_region| {
                other
                    .clone()
                    .0
                    .into_iter()
                    .map(|other_region| self_region.clone() - other_region)
                    .sum()
            })
            .sum()
    }
}

impl Sum for Regions {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |sum, region| sum + region)
    }
}

impl TryFrom<Range<usize>> for Regions {
    type Error = ();

    fn try_from(range: Range<usize>) -> Result<Self, Self::Error> {
        range.try_into().map(|range| Self(vec![range]))
    }
}

impl From<&[Range<usize>]> for Regions {
    fn from(ranges: &[Range<usize>]) -> Self {
        ranges
            .iter()
            .filter_map(|range| range.clone().try_into().ok())
            .sum()
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

    #[test]
    fn add_regions() {
        let a: Regions = [(0..1), (2..3), (4..5)].as_slice().into();
        let b: Regions = [(1..2), (3..4), (5..6)].as_slice().into();
        assert_eq!(a + b, (0..6).try_into().unwrap());
    }

    #[test]
    fn subtract_region() {
        let a: Region = (0..1).try_into().unwrap();
        let b: Region = (2..3).try_into().unwrap();
        assert_eq!(a.clone() - b.clone(), a.clone().into());
        assert_eq!(b.clone() - a.clone(), b.clone().into());
        let a: Region = (0..1).try_into().unwrap();
        let b: Region = (1..2).try_into().unwrap();
        assert_eq!(a.clone() - b.clone(), a.clone().into());
        assert_eq!(b.clone() - a.clone(), b.clone().into());
        let a: Region = (0..2).try_into().unwrap();
        let b: Region = (1..3).try_into().unwrap();
        assert_eq!(a.clone() - b.clone(), (0..1).try_into().unwrap());
        assert_eq!(b.clone() - a.clone(), (2..3).try_into().unwrap());
        let a: Region = (0..2).try_into().unwrap();
        let b: Region = (0..1).try_into().unwrap();
        assert_eq!(a.clone() - b.clone(), (1..2).try_into().unwrap());
        assert_eq!(b.clone() - a.clone(), Regions::default());
        let a: Region = (0..2).try_into().unwrap();
        let b: Region = (1..2).try_into().unwrap();
        assert_eq!(a.clone() - b.clone(), (0..1).try_into().unwrap());
        assert_eq!(b.clone() - a.clone(), Regions::default());
        let a: Region = (0..3).try_into().unwrap();
        let b: Region = (1..2).try_into().unwrap();
        assert_eq!(a.clone() - b.clone(), [(0..1), (2..3)].as_slice().into());
        assert_eq!(b.clone() - a.clone(), Regions::default());
    }
}
