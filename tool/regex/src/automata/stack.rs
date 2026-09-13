use {
    super::Automata,
    alloc::{vec, vec::Vec},
};

pub struct Frame<'a> {
    automata: &'a Automata,
    repetition_count: Option<usize>,
}

impl<'a> From<&'a Automata> for Frame<'a> {
    fn from(automata: &'a Automata) -> Self {
        Self {
            automata,
            repetition_count: if let Automata::Repetition { body, number } = automata {
                Some(0)
            } else {
                None
            },
        }
    }
}

pub struct Stack<'a>(Vec<Frame<'a>>);

impl<'a> From<&'a Automata> for Stack<'a> {
    fn from(automata: &'a Automata) -> Self {
        Self(vec![automata.into()])
    }
}
