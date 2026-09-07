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
pub struct Region<T: UnsignedInt>(Range<T>);

impl<T: UnsignedInt> Region<T> {
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

impl<T: UnsignedInt> Add for Region<T> {
    type Output = Regions<T>;

    fn add(self, other: Self) -> Self::Output {
        let left: Regions<T> = self.into();
        let right: Regions<T> = other.into();
        left + right
    }
}

impl<T: UnsignedInt> Debug for Region<T> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl<T: UnsignedInt> Sub for Region<T> {
    type Output = Regions<T>;

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

impl<T: UnsignedInt> TryFrom<Range<T>> for Region<T> {
    type Error = ();

    fn try_from(range: Range<T>) -> Result<Self, Self::Error> {
        if range.is_empty() {
            Err(())
        } else {
            Ok(Self(range))
        }
    }
}

#[derive(Clone, Default, Eq, PartialEq)]
pub struct Regions<T: UnsignedInt>(Vec<Region<T>>);

impl<T: UnsignedInt> Regions<T> {
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

    fn split_first(self) -> Option<(Region<T>, Self)> {
        self.0
            .split_first()
            .map(|(head, body)| (head.clone(), Self(body.iter().cloned().collect())))
    }
}

impl<T: UnsignedInt> Add for Regions<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut output: Self = Self(self.0.into_iter().chain(other.0).collect());
        output.normalize();
        output
    }
}

impl<T: UnsignedInt> Debug for Regions<T> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.debug_list().entries(self.0.iter()).finish()
    }
}

impl<T: UnsignedInt> From<Region<T>> for Regions<T> {
    fn from(region: Region<T>) -> Self {
        Self(vec![region])
    }
}

impl<T: UnsignedInt> Sub for Regions<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self.0
            .into_iter()
            .map(|self_region| {
                if let Some((other_region, other)) = other.clone().split_first() {
                    (self_region - other_region) - other
                } else {
                    self_region.into()
                }
            })
            .sum()
    }
}

impl<T: UnsignedInt> Sum for Regions<T> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |sum, region| sum + region)
    }
}

impl<T: UnsignedInt> TryFrom<Range<T>> for Regions<T> {
    type Error = ();

    fn try_from(range: Range<T>) -> Result<Self, Self::Error> {
        range.try_into().map(|range| Self(vec![range]))
    }
}

impl<T: UnsignedInt> From<&[Range<T>]> for Regions<T> {
    fn from(ranges: &[Range<T>]) -> Self {
        ranges
            .iter()
            .filter_map(|range| range.clone().try_into().ok())
            .sum()
    }
}

trait UnsignedInt:
    Add + Clone + Copy + Debug + Default + Eq + Ord + PartialEq + PartialOrd + Sub
{
}

impl UnsignedInt for i8 {}
impl UnsignedInt for i16 {}
impl UnsignedInt for i32 {}
impl UnsignedInt for i64 {}
impl UnsignedInt for i128 {}
impl UnsignedInt for isize {}
impl UnsignedInt for u8 {}
impl UnsignedInt for u16 {}
impl UnsignedInt for u32 {}
impl UnsignedInt for u64 {}
impl UnsignedInt for u128 {}
impl UnsignedInt for usize {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn region_try_merge() {
        let a: Region<usize> = (0..1).try_into().unwrap();
        let b: Region<usize> = (2..3).try_into().unwrap();
        assert_eq!(a.try_merge(&b), None);
        assert_eq!(b.try_merge(&a), None);
        let a: Region<usize> = (0..1).try_into().unwrap();
        let b: Region<usize> = (1..2).try_into().unwrap();
        assert_eq!(a.try_merge(&b), (0..2).try_into().ok());
        assert_eq!(b.try_merge(&a), (0..2).try_into().ok());
        let a: Region<usize> = (0..2).try_into().unwrap();
        let b: Region<usize> = (1..3).try_into().unwrap();
        assert_eq!(a.try_merge(&b), (0..3).try_into().ok());
        assert_eq!(b.try_merge(&a), (0..3).try_into().ok());
        let a: Region<usize> = (0..2).try_into().unwrap();
        let b: Region<usize> = (0..1).try_into().unwrap();
        assert_eq!(a.try_merge(&b), (0..2).try_into().ok());
        assert_eq!(b.try_merge(&a), (0..2).try_into().ok());
        let a: Region<usize> = (0..2).try_into().unwrap();
        let b: Region<usize> = (1..2).try_into().unwrap();
        assert_eq!(a.try_merge(&b), (0..2).try_into().ok());
        assert_eq!(b.try_merge(&a), (0..2).try_into().ok());
        let a: Region<usize> = (0..3).try_into().unwrap();
        let b: Region<usize> = (1..2).try_into().unwrap();
        assert_eq!(a.try_merge(&b), (0..3).try_into().ok());
        assert_eq!(b.try_merge(&a), (0..3).try_into().ok());
    }

    #[test]
    fn add_regions() {
        let a: Regions<usize> = [(0..1), (2..3), (4..5)].as_slice().into();
        let b: Regions<usize> = [(1..2), (3..4), (5..6)].as_slice().into();
        assert_eq!(a + b, (0..6).try_into().unwrap());
    }

    #[test]
    fn subtract_region() {
        let a: Region<usize> = (0..1).try_into().unwrap();
        let b: Region<usize> = (2..3).try_into().unwrap();
        assert_eq!(a.clone() - b.clone(), a.clone().into());
        assert_eq!(b.clone() - a.clone(), b.clone().into());
        let a: Region<usize> = (0..1).try_into().unwrap();
        let b: Region<usize> = (1..2).try_into().unwrap();
        assert_eq!(a.clone() - b.clone(), a.clone().into());
        assert_eq!(b.clone() - a.clone(), b.clone().into());
        let a: Region<usize> = (0..2).try_into().unwrap();
        let b: Region<usize> = (1..3).try_into().unwrap();
        assert_eq!(a.clone() - b.clone(), (0..1).try_into().unwrap());
        assert_eq!(b.clone() - a.clone(), (2..3).try_into().unwrap());
        let a: Region<usize> = (0..2).try_into().unwrap();
        let b: Region<usize> = (0..1).try_into().unwrap();
        assert_eq!(a.clone() - b.clone(), (1..2).try_into().unwrap());
        assert_eq!(b.clone() - a.clone(), Regions::<usize>::default());
        let a: Region<usize> = (0..2).try_into().unwrap();
        let b: Region<usize> = (1..2).try_into().unwrap();
        assert_eq!(a.clone() - b.clone(), (0..1).try_into().unwrap());
        assert_eq!(b.clone() - a.clone(), Regions::<usize>::default());
        let a: Region<usize> = (0..3).try_into().unwrap();
        let b: Region<usize> = (1..2).try_into().unwrap();
        assert_eq!(a.clone() - b.clone(), [(0..1), (2..3)].as_slice().into());
        assert_eq!(b.clone() - a.clone(), Regions::<usize>::default());
    }

    #[test]
    fn subtract_regions() {
        let a: Regions<usize> = [0..5, 6..11].as_slice().into();
        let b: Regions<usize> = [1..2, 3..4, 7..8, 9..10].as_slice().into();
        assert_eq!(
            a - b,
            [0..1, 2..3, 4..5, 6..7, 8..9, 10..11].as_slice().into()
        );
    }
}
