pub mod transition;

use {
    super::{Automaton, Stack},
    crate::search::Character,
    alloc::vec::Vec,
};

#[derive(Clone, Debug)]
pub struct Acceptance<'a> {
    index: usize,
    stack: Stack<'a>,
}

impl<'a> Acceptance<'a> {
    pub fn accept(stack: &Stack<'a>, character: &Character) -> Option<Self> {
        stack
            .acceptor()
            .accepts(character.character())
            .then_some(Self {
                index: character.index(),
                stack: stack.clone(),
            })
    }

    pub fn accepted(&self) -> bool {
        self.stack.accepted()
    }

    pub fn accepted_in(&self, stack: &Stack) -> bool {
        self.stack.is_based_on(stack)
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn stack_history(previous: Option<&Self>, next: Option<&Self>) -> Vec<Stack<'a>> {
        previous
            .map_or_default(|previous| previous.stack.clone())
            .history_to(&next.map_or_default(|next| next.stack.clone()))
    }
}
