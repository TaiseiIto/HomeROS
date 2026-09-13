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

impl From<char> for Automata {
    fn from(character: char) -> Self {
        Self::Character(character.into())
    }
}

impl FromStr for Automata {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        string
            .parse()
            .map(|expression: Expression| expression.into())
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
