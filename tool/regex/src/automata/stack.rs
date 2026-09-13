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
    progress: Progress,
}

impl<'a> From<&'a Automata> for Frame<'a> {
    fn from(automata: &'a Automata) -> Self {
        Self {
            automata,
            progress: automata.into(),
        }
    }
}

enum Progress {
    Character,
    EndOfLine,
    Repetition { repetition_count: usize },
    Selection,
    Sequence { processing_element_index: usize },
    StartOfLine,
}

impl From<&Automata> for Progress {
    fn from(automata: &Automata) -> Self {
        match automata {
            Automata::Character(_) => Self::Character,
            Automata::EndOfLine => Self::EndOfLine,
            Automata::Repetition { body: _, number: _ } => Self::Repetition {
                repetition_count: 0,
            },
            Automata::Selection(_) => Self::Selection,
            Automata::Sequence(_) => Self::Sequence {
                processing_element_index: 0,
            },
            Automata::StartOfLine => Self::StartOfLine,
        }
    }
}
