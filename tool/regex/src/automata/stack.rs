use {super::Automata, alloc::vec::Vec};

pub struct Frame<'a> {
    automata: &'a Automata,
    repetition_count: Option<usize>,
}

pub struct Stack<'a>(Vec<Frame<'a>>);
