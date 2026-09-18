use {
    super::{super::Stack, Acceptance},
    crate::search::Point,
    alloc::{vec, vec::Vec},
    core::{iter::once, ops::RangeInclusive},
};

#[derive(Debug)]
pub struct Line<'a>(Vec<Acceptance<'a>>);

impl Line<'_> {
    pub fn accepted(&self) -> bool {
        self.0
            .last()
            .is_some_and(|acceptance| acceptance.accepted())
    }

    pub fn index_range(&self, stack: &Stack) -> RangeInclusive<usize> {
        let indices: Vec<usize> = self
            .0
            .iter()
            .filter(|acceptance| acceptance.accepted_in(stack))
            .map(|acceptance| acceptance.index())
            .collect();
        let max: usize = *indices.iter().max().unwrap();
        let min: usize = *indices.iter().min().unwrap();
        min..=max
    }
}

impl<'a> Line<'a> {
    pub fn captures(&'a self) -> Vec<Stack<'a>> {
        once(None)
            .chain(self.0.iter().map(Some))
            .zip(self.0.iter().map(Some).chain(once(None)))
            .flat_map(|(previous_acceptance, next_acceptance)| {
                Acceptance::stack_history(previous_acceptance, next_acceptance)
                    .into_iter()
                    .skip(1)
            })
            .filter(|stack| stack.is_start_of_capture())
            .collect()
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
