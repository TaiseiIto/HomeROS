use {
    crate::automata::{input::Character, state},
    alloc::{collections::btree_set::BTreeSet, vec::Vec},
    core::{
        iter::{Sum, once},
        ops::{Add, Neg, Not, Sub},
    },
};

#[derive(Debug, Default)]
pub struct Acceptor {
    acceptance: Acceptance,
    characters: BTreeSet<char>,
}

impl Acceptor {
    pub fn accept<'a>(&'a self, character: &Character) -> Option<state::Acceptance<'a>> {
        state::Acceptance::accept(self, character)
    }

    pub fn accepts(&self, character: char) -> bool {
        match self {
            Self {
                acceptance: Acceptance::Set,
                characters,
            } => characters.contains(&character),
            Self {
                acceptance: Acceptance::Complement,
                characters,
            } => !characters.contains(&character),
        }
    }
}

impl Add for Acceptor {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let Self {
            acceptance: self_acceptance,
            characters: self_characters,
        } = self;
        let Self {
            acceptance: other_acceptance,
            characters: other_characters,
        } = other;
        match (self_acceptance, other_acceptance) {
            (Acceptance::Set, Acceptance::Set) => Self {
                acceptance: Acceptance::Set,
                characters: &self_characters | &other_characters,
            },
            (Acceptance::Set, Acceptance::Complement) => Self {
                acceptance: Acceptance::Complement,
                characters: &other_characters - &self_characters,
            },
            (Acceptance::Complement, Acceptance::Set) => Self {
                acceptance: Acceptance::Complement,
                characters: &self_characters - &other_characters,
            },
            (Acceptance::Complement, Acceptance::Complement) => Self {
                acceptance: Acceptance::Complement,
                characters: self_characters
                    .intersection(&other_characters)
                    .cloned()
                    .collect(),
            },
        }
    }
}

impl Neg for Acceptor {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let Self {
            acceptance,
            characters,
        } = self;
        Self {
            acceptance: !acceptance,
            characters,
        }
    }
}

impl Sub for Acceptor {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self + (-other)
    }
}

impl Sum for Acceptor {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |sum, element| sum + element)
    }
}

impl From<char> for Acceptor {
    fn from(character: char) -> Self {
        once(character).collect()
    }
}

impl FromIterator<char> for Acceptor {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        Self {
            acceptance: Acceptance::Set,
            characters: iter.into_iter().collect(),
        }
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
