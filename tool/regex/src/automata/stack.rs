use {
    super::{Automata, search::Character, state::Acceptance},
    crate::character::Acceptor,
    alloc::{vec, vec::Vec},
};

#[derive(Clone, Debug)]
pub struct Stack<'a>(Vec<Frame<'a>>);

impl<'a> Stack<'a> {
    pub fn accept(&self, character: &Character) -> Option<Acceptance<'a>> {
        Acceptance::accept(self, character)
    }

    pub fn accepted(&self) -> bool {
        self.0.iter().all(|frame| frame.accepted())
    }

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

    pub fn next_states(&self, start_of_line: bool, end_of_line: bool) -> Vec<Self> {
        let mut next_stack: Self = self.clone();
        if let Some(frame) = next_stack.0.last_mut() {
            match frame {
                Frame {
                    automata: Automata::Character(_),
                    progress: Progress::Character { executed },
                } => {
                    if *executed {
                        next_stack.0.pop();
                        next_stack.next_states(start_of_line, end_of_line)
                    } else {
                        *executed = true;
                        vec![next_stack]
                    }
                }
                Frame {
                    automata: Automata::EndOfLine,
                    progress: Progress::EndOfLine,
                } => {
                    if end_of_line {
                        next_stack.0.pop();
                        next_stack.next_states(start_of_line, end_of_line)
                    } else {
                        Vec::default()
                    }
                }
                Frame {
                    automata: Automata::Repetition { body, number },
                    progress: Progress::Repetition { repetition_count },
                } => {
                    let repetition_count: usize = *repetition_count;
                    [
                        number.can_break(repetition_count).then_some({
                            let mut next_stack: Self = next_stack.clone();
                            next_stack.0.pop();
                            next_stack.next_states(start_of_line, end_of_line)
                        }),
                        number.can_continue(repetition_count).then_some({
                            let mut next_stack: Self = next_stack.clone();
                            if let Frame {
                                automata: _,
                                progress: Progress::Repetition { repetition_count },
                            } = next_stack.0.last_mut().unwrap()
                            {
                                *repetition_count += 1;
                                next_stack.0.push(Frame::initialize(body));
                                next_stack.next_states(start_of_line, end_of_line)
                            } else {
                                panic!();
                            }
                        }),
                    ]
                    .into_iter()
                    .flatten()
                    .flatten()
                    .collect()
                }
                Frame {
                    automata: Automata::Selection(options),
                    progress: Progress::Selection { executed },
                } => {
                    if *executed {
                        next_stack.0.pop();
                        next_stack.next_states(start_of_line, end_of_line)
                    } else {
                        *executed = true;
                        options
                            .iter()
                            .flat_map(|option| {
                                let mut next_stack: Self = next_stack.clone();
                                next_stack.0.push(Frame::initialize(option));
                                next_stack.next_states(start_of_line, end_of_line)
                            })
                            .collect()
                    }
                }
                Frame {
                    automata: Automata::Sequence(elements),
                    progress:
                        Progress::Sequence {
                            processing_element_index,
                        },
                } => {
                    if *processing_element_index < elements.len() {
                        let next_frame: Frame =
                            Frame::initialize(&elements[*processing_element_index]);
                        *processing_element_index += 1;
                        next_stack.0.push(next_frame);
                        next_stack.next_states(start_of_line, end_of_line)
                    } else {
                        next_stack.0.pop();
                        next_stack.next_states(start_of_line, end_of_line)
                    }
                }
                Frame {
                    automata: Automata::StartOfLine,
                    progress: Progress::StartOfLine,
                } => {
                    if start_of_line {
                        next_stack.0.pop();
                        next_stack.next_states(start_of_line, end_of_line)
                    } else {
                        Vec::default()
                    }
                }
                _ => panic!(),
            }
        } else {
            Vec::default()
        }
    }
}

#[derive(Clone, Debug)]
struct Frame<'a> {
    automata: &'a Automata,
    progress: Progress,
}

impl<'a> Frame<'a> {
    fn accepted(&self) -> bool {
        match self {
            Self {
                automata: _,
                progress: Progress::Character { executed },
            } => *executed,
            Self {
                automata: Automata::Repetition { body, number },
                progress: Progress::Repetition { repetition_count },
            } => number.can_break(*repetition_count) || body.accepts_empty_string(),
            Self {
                automata,
                progress: Progress::Selection { executed },
            } => *executed || automata.accepts_empty_string(),
            Self {
                automata: Automata::Sequence(elements),
                progress:
                    Progress::Sequence {
                        processing_element_index,
                    },
            } => elements
                .iter()
                .skip(*processing_element_index)
                .all(|element| element.accepts_empty_string()),
            _ => panic!(),
        }
    }

    fn initialize(automata: &'a Automata) -> Self {
        Self {
            automata,
            progress: Progress::initialize(automata),
        }
    }
}

#[derive(Clone, Debug)]
enum Progress {
    Character { executed: bool },
    EndOfLine,
    Repetition { repetition_count: usize },
    Selection { executed: bool },
    Sequence { processing_element_index: usize },
    StartOfLine,
}

impl Progress {
    fn initialize(automata: &Automata) -> Self {
        match automata {
            Automata::Character(_) => Self::Character { executed: false },
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
