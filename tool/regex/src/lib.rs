#![no_std]

extern crate alloc;

mod character;
mod symbol;

use {
    alloc::{boxed::Box, vec::Vec},
    core::str::FromStr,
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let _: Automata = r"^\d(\l+|\u*)[^a-d0-3_{}]{2,3}[^\w()*]{3}$"
            .parse()
            .unwrap();
    }
}
