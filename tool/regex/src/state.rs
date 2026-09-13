use {
    crate::{Automata, character},
    alloc::vec::Vec,
};

pub struct Transition<'a> {
    character_acceptance: CharacterAcceptance<'a>,
    next: Vec<Self>,
}

struct CharacterAcceptance<'a> {
    acceptor: &'a character::Set,
    character: char,
    index: usize,
}
