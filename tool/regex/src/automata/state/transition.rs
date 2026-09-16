use {
    super::{
        super::{Stack, search::Point},
        Acceptance,
    },
    alloc::vec::Vec,
};

#[derive(Debug)]
pub struct Tree<'a> {
    acceptance: Acceptance<'a>,
    next: Vec<Self>,
}

impl<'a> Tree<'a> {
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
