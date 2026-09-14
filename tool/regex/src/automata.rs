pub mod input;
mod stack;
pub mod state;

use {
    crate::{character::Acceptor, symbol::Expression},
    alloc::{boxed::Box, vec::Vec},
    core::str::FromStr,
};

pub use stack::Stack;

#[derive(Debug)]
pub enum Automata {
    Character(Acceptor),
    EndOfLine,
    Repetition {
        body: Box<Self>,
        number: RepetitionNumber,
    },
    Selection(Vec<Self>),
    Sequence(Vec<Self>),
    StartOfLine,
}

impl Automata {
    pub fn input<'a>(&'a self, input: &'a str) -> Vec<state::Transition<'a>> {
        state::Transition::execute(Stack::initialize(self), input.into())
    }
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

#[derive(Debug)]
pub enum RepetitionNumber {
    Constant(usize),
    From(usize),
    FromTo(usize, usize),
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
