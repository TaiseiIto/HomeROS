use {
    crate::{Automaton, automaton::state::transition::Line},
    alloc::vec::Vec,
};

#[derive(Debug)]
pub struct Capture<'a> {
    mat: &'a Match<'a>,
    automaton: &'a Automaton,
}

impl<'a> From<&'a Match<'a>> for Vec<Capture<'a>> {
    fn from(mat: &'a Match<'a>) -> Self {
        mat.automaton_state_transition
            .first_call_ordered_automata()
            .into_iter()
            .filter(|automaton| matches!(automaton, Automaton::Capturer(_)))
            .map(|automaton| Capture { mat, automaton })
            .collect()
    }
}

impl<'a> From<&'a Capture<'a>> for &'a str {
    fn from(capture: &'a Capture<'a>) -> Self {
        let Capture { mat, automaton } = capture;
        mat.capture(automaton)
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

    fn capture(&'a self, automaton: &'a Automaton) -> &'a str {
        let Self {
            input,
            automaton_state_transition,
        } = self;
        &input[automaton_state_transition.index_range(automaton)]
    }
}
