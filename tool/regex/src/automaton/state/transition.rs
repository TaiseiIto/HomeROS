use {
    super::{super::Stack, Acceptance},
    crate::{Automaton, search::Point},
    alloc::{vec, vec::Vec},
    core::iter::once,
    core::ptr::eq,
};

#[derive(Debug)]
pub struct Line<'a>(Vec<Acceptance<'a>>);

impl Line<'_> {
    pub fn accepted(&self) -> bool {
        self.0
            .last()
            .is_some_and(|acceptance| acceptance.accepted())
    }
}

impl<'a> Line<'a> {
    pub fn first_call_ordered_automata(&'a self) -> Vec<&'a Automaton> {
        self.0
            .iter()
            .flat_map(|acceptance| acceptance.automaton_layers().into_iter())
            .fold(Vec::default(), |mut automata, new_automaton| {
                if automata.iter().all(|automaton| {
                    !eq(
                        *automaton as *const Automaton,
                        new_automaton as *const Automaton,
                    )
                }) {
                    automata.push(new_automaton);
                }
                automata
            })
    }
}

impl<'a> From<Tree<'a>> for Vec<Line<'a>> {
    fn from(tree: Tree<'a>) -> Self {
        let Tree { acceptance, next } = tree;
        if next.is_empty() {
            vec![Line(vec![acceptance])]
        } else {
            next.into_iter()
                .flat_map(|next| {
                    let next: Vec<Line<'a>> = next.into();
                    next.into_iter()
                        .map(|next| Line(once(acceptance.clone()).chain(next.0).collect()))
                })
                .collect()
        }
    }
}

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
                    stack.accept(&character).map(|acceptance| Self {
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
