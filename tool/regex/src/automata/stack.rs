use {
    super::Automata,
    crate::character::Acceptor,
    alloc::{vec, vec::Vec},
};

pub struct Stack<'a>(Vec<Frame<'a>>);

impl<'a> Stack<'a> {
    pub fn acceptor(&self) -> &'a Acceptor {
        if let Automata::Character(acceptor) = self.0.last().unwrap().automata {
            &acceptor
        } else {
            panic!();
        }
    }

    pub fn initialize(automata: &'a Automata) -> Self {
        Self(vec![Frame::initialize(automata)])
    }

    pub fn next_states(&self) -> Vec<Self> {
        unimplemented!();
    }
}

struct Frame<'a> {
    automata: &'a Automata,
    progress: Progress,
}

impl<'a> Frame<'a> {
    fn initialize(automata: &'a Automata) -> Self {
        Self {
            automata,
            progress: Progress::initialize(automata),
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

impl Progress {
    fn initialize(automata: &Automata) -> Self {
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
