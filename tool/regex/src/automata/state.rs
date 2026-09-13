use {
    super::{Automata, Stack, input},
    crate::character,
    alloc::vec::Vec,
};

pub struct Transition<'a> {
    character_acceptance: CharacterAcceptance<'a>,
    next: Vec<Self>,
}

impl<'a> Transition<'a> {
    pub fn simulate(stack: Stack<'a>, input: input::String) -> Vec<Self> {
        unimplemented!();
    }
}

struct CharacterAcceptance<'a> {
    acceptor: &'a character::Set,
    character: char,
    index: usize,
}
