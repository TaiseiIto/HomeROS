use {
    super::{
        Automata, Stack,
        search::{Character, Point},
    },
    crate::character::Acceptor,
    alloc::vec::Vec,
};

#[derive(Debug)]
pub struct Transition<'a> {
    acceptance: Acceptance<'a>,
    next: Vec<Self>,
}

impl<'a> Transition<'a> {
    pub fn execute(stack: Stack<'a>, mut input: Point) -> Vec<Self> {
        if let Some(character) = input.next() {
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
    pub fn accept(acceptor: &'a Acceptor, character: &Character) -> Option<Self> {
        acceptor.accepts(character.character()).then_some(Self {
            acceptor,
            character: character.character(),
            index: character.index(),
        })
    }
}
