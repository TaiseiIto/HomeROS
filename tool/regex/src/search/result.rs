use crate::automaton::state::transition::Line;

#[derive(Debug)]
pub struct Match<'a> {
    input: &'a str,
    automaton_state_transition: Line<'a>,
}

impl<'a> Match<'a> {
    pub fn new(input: &'a str, automaton_state_transition: Line<'a>) -> Self {
        Self {
            input,
            automaton_state_transition,
        }
    }
}
