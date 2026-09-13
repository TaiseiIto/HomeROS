use {
    alloc::{collections::btree_set::BTreeSet, vec::Vec},
    core::{
        iter::{Sum, once},
        ops::{Add, Neg, Not, Sub},
    },
};

#[derive(Debug, Default)]
pub struct Set {
    acceptance: Acceptance,
    subsets: Vec<Subset>,
}

impl Add for Set {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            acceptance: Acceptance::Set,
            subsets: [self, other].into_iter().map(Subset::Set).collect(),
        }
    }
}

impl Neg for Set {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let Self {
            acceptance,
            subsets,
        } = self;
        Self {
            acceptance: !acceptance,
            subsets,
        }
    }
}

impl Sub for Set {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self + (-other)
    }
}

impl Sum for Set {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |sum, element| sum + element)
    }
}

impl From<char> for Set {
    fn from(character: char) -> Self {
        once(character).collect()
    }
}

impl FromIterator<char> for Set {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        Self {
            acceptance: Acceptance::Set,
            subsets: once(iter.into_iter().collect()).collect(),
        }
    }
}

#[derive(Debug)]
enum Subset {
    Set(Set),
    Characters(BTreeSet<char>),
}

impl From<char> for Subset {
    fn from(character: char) -> Self {
        once(character).collect()
    }
}

impl FromIterator<char> for Subset {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        Self::Characters(iter.into_iter().collect())
    }
}

#[derive(Debug)]
enum Acceptance {
    Complement,
    Set,
}

impl Default for Acceptance {
    fn default() -> Self {
        Self::Set
    }
}

impl Not for Acceptance {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Complement => Self::Set,
            Self::Set => Self::Complement,
        }
    }
}
