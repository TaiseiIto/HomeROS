pub mod search;
mod stack;
pub mod state;

use {
    crate::{character::Acceptor, symbol::Expression},
    alloc::{boxed::Box, vec::Vec},
    core::str::FromStr,
    search::Target,
    state::transition::{Line, Tree},
};

pub use stack::Stack;

#[derive(Debug)]
pub enum Automata {
    Character(Acceptor),
    EndOfLine,
    Repetition {
        body: Box<Self>,
        number: RepetitionNumber,
    },
    Selection(Vec<Self>),
    Sequence(Vec<Self>),
    StartOfLine,
}

impl Automata {
    pub fn accepts_empty_string(&self) -> bool {
        match self {
            Self::Character(_) => false,
            Self::EndOfLine => true,
            Self::Repetition { body, number } => number.can_break(0) || body.accepts_empty_string(),
            Self::Selection(options) => options.iter().any(|option| option.accepts_empty_string()),
            Self::Sequence(elements) => elements
                .iter()
                .all(|element| element.accepts_empty_string()),
            Self::StartOfLine => true,
        }
    }

    pub fn input<'a>(&'a self, input: &str) -> Vec<Line<'a>> {
        let mut input: Target = input.into();
        let trees: Vec<Tree<'a>> = input
            .flat_map(|search_point| Tree::execute(Stack::initialize(self), search_point))
            .collect();
        trees
            .into_iter()
            .flat_map(Into::<Vec<Line<'a>>>::into)
            .filter(|line| line.accepted())
            .collect()
    }
}

impl From<char> for Automata {
    fn from(character: char) -> Self {
        Self::Character(character.into())
    }
}

impl FromStr for Automata {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        string
            .parse()
            .map(|expression: Expression| expression.into())
    }
}

#[derive(Debug)]
pub enum RepetitionNumber {
    Constant(usize),
    From(usize),
    FromTo(usize, usize),
}

impl RepetitionNumber {
    pub fn can_continue(&self, current: usize) -> bool {
        match self {
            Self::Constant(constant) => current < *constant,
            Self::From(_) => true,
            Self::FromTo(_, to) => current < *to,
        }
    }

    pub fn can_break(&self, current: usize) -> bool {
        match self {
            Self::Constant(constant) => *constant <= current,
            Self::From(from) => *from <= current,
            Self::FromTo(from, _) => *from <= current,
        }
    }
}
