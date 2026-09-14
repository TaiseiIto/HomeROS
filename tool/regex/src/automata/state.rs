use {
    super::{Automata, Stack, input},
    crate::character::Acceptor,
    alloc::vec::Vec,
};

pub struct Transition<'a> {
    acceptance: Acceptance<'a>,
    next: Vec<Self>,
}

impl<'a> Transition<'a> {
    pub fn simulate(stack: Stack<'a>, mut input: input::String) -> Vec<Self> {
        if let Some(character) = input.pop_front() {
            stack
                .next_states()
                .into_iter()
                .filter_map(|stack| {
                    stack.acceptor().accept(&character).map(|acceptance| Self {
                        acceptance,
                        next: Self::simulate(stack, input.clone()),
                    })
                })
                .collect()
        } else {
            Vec::default()
        }
    }
}

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
