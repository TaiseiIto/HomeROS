use {
    super::{Automata, Stack, input},
    crate::character::Acceptor,
    alloc::vec::Vec,
};

#[derive(Debug)]
pub struct Transition<'a> {
    acceptance: Acceptance<'a>,
    next: Vec<Self>,
}

impl<'a> Transition<'a> {
    pub fn execute(stack: Stack<'a>, mut input: input::SearchPoint) -> Vec<Self> {
        if let Some(character) = input.pop_front() {
            stack
                .next_states(character.index() == 0, input.is_empty())
                .into_iter()
                .filter_map(|stack| {
                    stack.acceptor().accept(&character).map(|acceptance| Self {
                        acceptance,
                        next: Self::execute(stack, input.clone()),
                    })
                })
                .collect()
        } else {
            Vec::default()
        }
    }
}

#[derive(Debug)]
pub struct Acceptance<'a> {
    acceptor: &'a Acceptor,
    character: char,
    index: usize,
}

impl<'a> Acceptance<'a> {
    pub fn accept(acceptor: &'a Acceptor, character: &input::Character) -> Option<Self> {
        acceptor.accepts(character.character()).then_some(Self {
            acceptor,
            character: character.character(),
            index: character.index(),
        })
    }
}
