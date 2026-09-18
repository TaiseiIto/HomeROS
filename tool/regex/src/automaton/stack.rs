use {
    super::{Automaton, state::Acceptance},
    crate::{character::Acceptor, search::Character},
    alloc::{vec, vec::Vec},
    core::{
        cmp::Ordering::{Equal, Greater, Less},
        iter::once,
    },
};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Stack<'a>(Vec<Frame<'a>>);

impl<'a> Stack<'a> {
    pub fn accept(&self, character: &Character) -> Option<Acceptance<'a>> {
        Acceptance::accept(self, character)
    }

    pub fn accepted(&self) -> bool {
        self.0.iter().all(|frame| frame.accepted())
    }

    pub fn acceptor(&self) -> &'a Acceptor {
        if let Automaton::Character(acceptor) = self.0.last().unwrap().automaton {
            acceptor
        } else {
            panic!();
        }
    }

    pub fn automaton_layers(&'a self) -> Vec<&'a Automaton> {
        self.0.iter().map(|frame| frame.automaton).collect()
    }

    pub fn history_to(&self, next: &Self) -> Vec<Self> {
        if self == next {
            vec![self.clone()]
        } else {
            match self.0.len().cmp(&next.0.len()) {
                Equal => {
                    if self.0.is_empty() {
                        Vec::default()
                    } else {
                        let previous: Self = self.clone();
                        let mut popped_previous: Self = previous.clone();
                        popped_previous.0.pop().unwrap();
                        let next: Self = next.clone();
                        let mut popped_next: Self = next.clone();
                        popped_next.0.pop().unwrap();
                        once(previous)
                            .chain(popped_previous.history_to(&popped_next))
                            .chain(once(next))
                            .collect()
                    }
                }
                Greater => {
                    let previous: Self = self.clone();
                    let mut popped_previous: Self = previous.clone();
                    popped_previous.0.pop().unwrap();
                    once(previous)
                        .chain(popped_previous.history_to(next))
                        .collect()
                }
                Less => {
                    let next: Self = next.clone();
                    let mut popped_next: Self = next.clone();
                    popped_next.0.pop().unwrap();
                    self.history_to(&popped_next)
                        .into_iter()
                        .chain(once(next))
                        .collect()
                }
            }
        }
    }

    pub fn initialize(automaton: &'a Automaton) -> Self {
        Self(vec![Frame::initialize(automaton)])
    }

    pub fn is_based_on(&self, base: &Self) -> bool {
        self.0
            .iter()
            .zip(base.0.iter())
            .all(|(my_frame, base_frame)| my_frame == base_frame)
    }

    pub fn is_start_of_capture(&self) -> bool {
        self.0
            .last()
            .is_some_and(|frame| matches!(frame.automaton, Automaton::Capturer(_)))
    }

    pub fn next_states(&self, start_of_line: bool, end_of_line: bool) -> Vec<Self> {
        let mut next_stack: Self = self.clone();
        if let Some(frame) = next_stack.0.last_mut() {
            match frame {
                Frame {
                    automaton: Automaton::Capturer(capturer),
                    progress: Progress::Capturer { executed },
                } => {
                    if *executed {
                        next_stack.0.pop().unwrap();
                        next_stack.next_states(start_of_line, end_of_line)
                    } else {
                        *executed = true;
                        let mut next_stack: Self = next_stack.clone();
                        next_stack.0.push(Frame::initialize(capturer));
                        next_stack.next_states(start_of_line, end_of_line)
                    }
                }
                Frame {
                    automaton: Automaton::Character(_),
                    progress: Progress::Character { executed },
                } => {
                    if *executed {
                        next_stack.0.pop().unwrap();
                        next_stack.next_states(start_of_line, end_of_line)
                    } else {
                        *executed = true;
                        vec![next_stack]
                    }
                }
                Frame {
                    automaton: Automaton::EndOfLine,
                    progress: Progress::EndOfLine,
                } => {
                    if end_of_line {
                        next_stack.0.pop().unwrap();
                        next_stack.next_states(start_of_line, end_of_line)
                    } else {
                        Vec::default()
                    }
                }
                Frame {
                    automaton: Automaton::Repetition { body, number },
                    progress: Progress::Repetition { repetition_count },
                } => {
                    let repetition_count: usize = *repetition_count;
                    [
                        number.can_break(repetition_count).then_some({
                            let mut next_stack: Self = next_stack.clone();
                            next_stack.0.pop().unwrap();
                            next_stack.next_states(start_of_line, end_of_line)
                        }),
                        number.can_continue(repetition_count).then_some({
                            let mut next_stack: Self = next_stack.clone();
                            if let Frame {
                                automaton: _,
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
                    automaton: Automaton::Selection(options),
                    progress: Progress::Selection { executed },
                } => {
                    if *executed {
                        next_stack.0.pop().unwrap();
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
                    automaton: Automaton::Sequence(elements),
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
                        next_stack.0.pop().unwrap();
                        next_stack.next_states(start_of_line, end_of_line)
                    }
                }
                Frame {
                    automaton: Automaton::StartOfLine,
                    progress: Progress::StartOfLine,
                } => {
                    if start_of_line {
                        next_stack.0.pop().unwrap();
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

#[derive(Clone, Debug, Eq, PartialEq)]
struct Frame<'a> {
    automaton: &'a Automaton,
    progress: Progress,
}

impl<'a> Frame<'a> {
    fn accepted(&self) -> bool {
        match self {
            Self {
                automaton: _,
                progress: Progress::Capturer { executed },
            } => *executed,
            Self {
                automaton: _,
                progress: Progress::Character { executed },
            } => *executed,
            Self {
                automaton: Automaton::Repetition { body, number },
                progress: Progress::Repetition { repetition_count },
            } => number.can_break(*repetition_count) || body.accepts_empty_string(),
            Self {
                automaton,
                progress: Progress::Selection { executed },
            } => *executed || automaton.accepts_empty_string(),
            Self {
                automaton: Automaton::Sequence(elements),
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

    fn initialize(automaton: &'a Automaton) -> Self {
        Self {
            automaton,
            progress: Progress::initialize(automaton),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Progress {
    Capturer { executed: bool },
    Character { executed: bool },
    EndOfLine,
    Repetition { repetition_count: usize },
    Selection { executed: bool },
    Sequence { processing_element_index: usize },
    StartOfLine,
}

impl Progress {
    fn initialize(automaton: &Automaton) -> Self {
        match automaton {
            Automaton::Capturer(_) => Self::Capturer { executed: false },
            Automaton::Character(_) => Self::Character { executed: false },
            Automaton::EndOfLine => Self::EndOfLine,
            Automaton::Repetition { body: _, number: _ } => Self::Repetition {
                repetition_count: 0,
            },
            Automaton::Selection(_) => Self::Selection { executed: false },
            Automaton::Sequence(_) => Self::Sequence {
                processing_element_index: 0,
            },
            Automaton::StartOfLine => Self::StartOfLine,
        }
    }
}
