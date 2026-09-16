pub mod transition;

use {super::search::Character, crate::character::Acceptor};

#[derive(Clone, Debug)]
pub struct Acceptance<'a> {
    acceptor: &'a Acceptor,
    character: char,
    index: usize,
}

impl<'a> Acceptance<'a> {
    pub fn accept(acceptor: &'a Acceptor, character: &Character) -> Option<Self> {
        acceptor.accepts(character.character()).then_some(Self {
            acceptor,
            character: character.character(),
            index: character.index(),
        })
    }
}
