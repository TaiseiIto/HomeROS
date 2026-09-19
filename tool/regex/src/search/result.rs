use {
    crate::automaton::{Stack, state::transition::Line},
    alloc::{string::String, vec::Vec},
};

#[derive(Debug)]
pub struct Capture<'a> {
    name: String,
    mat: &'a Match<'a>,
    stack: Stack<'a>,
}

impl<'a> From<&'a Match<'a>> for Vec<Capture<'a>> {
    fn from(mat: &'a Match<'a>) -> Self {
        mat.automaton_state_transition
            .captures()
            .into_iter()
            .map(|(name, stack)| Capture { name, mat, stack })
            .collect()
    }
}

impl<'a> From<&'a Capture<'a>> for &'a str {
    fn from(capture: &'a Capture<'a>) -> Self {
        let Capture {
            name: _,
            mat,
            stack,
        } = capture;
        mat.capture(stack)
    }
}

#[derive(Debug)]
pub struct Match<'a> {
    input: &'a str,
    automaton_state_transition: Line<'a>,
}

impl<'a> Match<'a> {
    pub fn captures(&'a self) -> Vec<Capture<'a>> {
        self.into()
    }

    pub fn new(input: &'a str, automaton_state_transition: Line<'a>) -> Self {
        Self {
            input,
            automaton_state_transition,
        }
    }

    fn capture(&'a self, stack: &'a Stack) -> &'a str {
        let Self {
            input,
            automaton_state_transition,
        } = self;
        &input[automaton_state_transition.index_range(stack)]
    }
}
