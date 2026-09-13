use {
    crate::{Automata, automata, character},
    alloc::vec::Vec,
};

pub struct Transition<'a> {
    character_acceptance: CharacterAcceptance<'a>,
    next: Vec<Self>,
}

impl<'a> Transition<'a> {
    pub fn simulate(automata: automata::Stack<'a>, input: automata::input::String) -> Vec<Self> {
        unimplemented!();
    }
}

struct CharacterAcceptance<'a> {
    acceptor: &'a character::Set,
    character: char,
    index: usize,
}
