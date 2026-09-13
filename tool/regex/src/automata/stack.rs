use {
    super::Automata,
    alloc::{vec, vec::Vec},
};

pub struct Stack<'a>(Vec<Frame<'a>>);

impl<'a> From<&'a Automata> for Stack<'a> {
    fn from(automata: &'a Automata) -> Self {
        Self(vec![automata.into()])
    }
}

struct Frame<'a> {
    automata: &'a Automata,
    phase: Phase,
}

impl<'a> From<&'a Automata> for Frame<'a> {
    fn from(automata: &'a Automata) -> Self {
        Self {
            automata,
            phase: Phase::BeforeStart,
        }
    }
}

enum Phase {
    BeforeStart,
    Processing(Progress),
    AfterEnd,
}

enum Progress {
    Character,
    EndOfLine,
    Repetition { repetition_count: usize },
    Selection,
    Sequence { processing_element_index: usize },
    StartOfLine,
}
