#![no_std]

extern crate alloc;

mod automata;
mod character;
pub mod search;
mod symbol;

pub use automata::Automata;

use automata::state::transition::Line;

#[derive(Debug)]
pub struct Match<'a> {
    input: &'a str,
    automata_state_transition: Line<'a>,
}

impl<'a> Match<'a> {
    pub fn new(input: &'a str, automata_state_transition: Line<'a>) -> Self {
        Self {
            input,
            automata_state_transition,
        }
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
