#![no_std]

extern crate alloc;

mod symbol;

use alloc::{boxed::Box, collections::btree_set::BTreeSet, vec::Vec};

pub enum Acceptance {
    Complement,
    Set,
}

pub enum Automata {
    Character {
        set: BTreeSet<char>,
        acceptance: Acceptance,
    },
    Repetition {
        body: Box<Automata>,
        min: usize,
        max: Option<usize>,
    },
    Selection(Vec<Automata>),
    Sequence(Vec<Automata>),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        symbol::Expression::parse(r"^\d(\l+|\u*)\w{2,3}$");
    }
}
