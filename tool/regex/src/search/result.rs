use {
    crate::automaton::{Stack, state::transition::Line},
    alloc::{collections::btree_map::BTreeMap, format, string::String, vec, vec::Vec},
    core::fmt::{Debug, Formatter, Result},
};

pub struct Capture<'a> {
    mat: &'a Match<'a>,
    stack: Stack<'a>,
}

impl Debug for Capture<'_> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        let capture: &str = self.into();
        formatter.write_str(&format!("{:#x?}", capture))
    }
}

impl<'a> From<&'a Match<'a>> for BTreeMap<String, Vec<Capture<'a>>> {
    fn from(mat: &'a Match<'a>) -> Self {
        mat.automaton_state_transition.captures().into_iter().fold(
            BTreeMap::new(),
            |mut name2captures, (name, stack)| {
                let capture: Capture = Capture { mat, stack };
                match name2captures.get_mut(&name) {
                    Some(captures) => {
                        captures.push(capture);
                    }
                    None => {
                        name2captures.insert(name, vec![capture]);
                    }
                }
                name2captures
            },
        )
    }
}

impl<'a> From<&'a Capture<'a>> for &'a str {
    fn from(capture: &'a Capture<'a>) -> Self {
        let Capture { mat, stack } = capture;
        mat.capture(stack)
    }
}

#[derive(Debug)]
pub struct Match<'a> {
    input: &'a str,
    automaton_state_transition: Line<'a>,
}

impl<'a> Match<'a> {
    pub fn captures(&'a self) -> BTreeMap<String, Vec<Capture<'a>>> {
        self.into()
    }

    pub fn new(input: &'a str, automaton_state_transition: Line<'a>) -> Self {
        Self {
            input,
            automaton_state_transition,
        }
    }

    fn capture(&'a self, stack: &'a Stack) -> &'a str {
        let Self {
            input,
            automaton_state_transition,
        } = self;
        &input[automaton_state_transition.index_range(stack)]
    }
}
