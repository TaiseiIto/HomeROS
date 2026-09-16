pub mod transition;

use super::{Stack, search::Character};

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
}
