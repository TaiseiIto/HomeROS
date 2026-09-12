#![no_std]

extern crate alloc;

mod character;
mod symbol;

use {
    alloc::{boxed::Box, collections::btree_set::BTreeSet, vec::Vec},
    core::{iter::once, str::FromStr},
    symbol::Expression,
};

pub enum Automata {
    Character(character::Set),
    EndOfLine,
    Repetition {
        body: Box<Automata>,
        min: usize,
        max: Option<usize>,
    },
    Selection(Vec<Automata>),
    Sequence(Vec<Automata>),
    StartOfLine,
}

impl FromStr for Automata {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        string
            .parse()
            .map(|expression: Expression| expression.into())
    }
}

impl<T> From<T> for Automata
where
    T: Into<char>,
{
    fn from(character: T) -> Self {
        Self::Character(character.into().into())
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//
//     #[test]
//     fn test() {
//         let _: Automata = r"^\d(\l+|\u*)\w{2,3}$".parse().unwrap();
//     }
// }
