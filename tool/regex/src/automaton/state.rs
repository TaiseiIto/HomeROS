pub mod transition;

use {
    super::{Automaton, Stack},
    crate::search::Character,
    alloc::vec::Vec,
};

#[derive(Clone, Debug)]
pub struct Acceptance<'a> {
    character: char,
    index: usize,
    stack: Stack<'a>,
}

impl<'a> Acceptance<'a> {
    pub fn accept(stack: &Stack<'a>, character: &Character) -> Option<Self> {
        stack
            .acceptor()
            .accepts(character.character())
            .then_some(Self {
                character: character.character(),
                index: character.index(),
                stack: stack.clone(),
            })
    }

    pub fn accepted(&self) -> bool {
        self.stack.accepted()
    }

    pub fn accepted_in(&self, automaton: &Automaton) -> bool {
        self.stack.contains(automaton)
    }

    pub fn automaton_layers(&'a self) -> Vec<&'a Automaton> {
        self.stack.automaton_layers()
    }

    pub fn index(&self) -> usize {
        self.index
    }
}
