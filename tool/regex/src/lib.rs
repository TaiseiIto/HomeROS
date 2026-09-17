#![no_std]

extern crate alloc;

mod automaton;
mod character;
pub mod search;
mod symbol;

pub use {
    automaton::Automaton,
    search::result::{Capture, Match},
};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let _: Automaton = r"^\d(\l+|\u*)[^a-d0-3_{}]{2,3}[^\w()*]{3}$"
            .parse()
            .unwrap();
    }
}
