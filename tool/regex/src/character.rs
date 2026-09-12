use {
    alloc::{collections::btree_set::BTreeSet, vec::Vec},
    core::{
        iter::{Sum, once},
        ops::{Add, Neg, Not, Sub},
    },
};

#[derive(Default)]
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

impl<T> From<T> for Set
where
    T: Into<char>,
{
    fn from(character: T) -> Self {
        Self {
            acceptance: Acceptance::Set,
            subsets: once(character.into().into()).collect(),
        }
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

enum Subset {
    Set(Set),
    Characters(BTreeSet<char>),
}

impl<T> From<T> for Subset
where
    T: Into<char>,
{
    fn from(character: T) -> Self {
        Self::Characters(once(character.into()).collect())
    }
}

impl FromIterator<char> for Subset {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        Self::Characters(iter.into_iter().collect())
    }
}

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
