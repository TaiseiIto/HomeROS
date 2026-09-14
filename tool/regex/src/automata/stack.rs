use {
    super::Automata,
    crate::character::Acceptor,
    alloc::{vec, vec::Vec},
};

#[derive(Clone)]
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
        if let Some(frame) = self.0.last() {
            match frame {
                Frame {
                    automata: Automata::Character(acceptor),
                    progress: Progress::Character,
                } => self.popped().next_states(),
                Frame {
                    automata: Automata::EndOfLine,
                    progress: Progress::EndOfLine,
                } => self.popped().next_states(),
                Frame {
                    automata: Automata::Repetition { body, number },
                    progress: Progress::Repetition { repetition_count },
                } => unimplemented!(),
                Frame {
                    automata: Automata::Selection(options),
                    progress: Progress::Selection { executed },
                } => unimplemented!(),
                Frame {
                    automata: Automata::Sequence(elements),
                    progress:
                        Progress::Sequence {
                            processing_element_index,
                        },
                } => unimplemented!(),
                Frame {
                    automata: Automata::StartOfLine,
                    progress: Progress::StartOfLine,
                } => self.popped().next_states(),
                _ => panic!(),
            }
        } else {
            Vec::default()
        }
    }

    fn popped(&self) -> Self {
        Self(self.0.iter().rev().skip(1).rev().cloned().collect())
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
enum Progress {
    Character,
    EndOfLine,
    Repetition { repetition_count: usize },
    Selection { executed: bool },
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
            Automata::Selection(_) => Self::Selection { executed: false },
            Automata::Sequence(_) => Self::Sequence {
                processing_element_index: 0,
            },
            Automata::StartOfLine => Self::StartOfLine,
        }
    }
}
