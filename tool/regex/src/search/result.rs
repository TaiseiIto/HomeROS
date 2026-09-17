use crate::automata::state::transition::Line;

#[derive(Debug)]
pub struct Match<'a> {
    input: &'a str,
    automata_state_transition: Line<'a>,
}

impl<'a> Match<'a> {
    pub fn new(input: &'a str, automata_state_transition: Line<'a>) -> Self {
        Self {
            input,
            automata_state_transition,
        }
    }
}
