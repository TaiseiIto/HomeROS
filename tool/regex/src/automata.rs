use {
    crate::symbol,
    alloc::{boxed::Box, collections::btree_set::BTreeSet, vec::Vec},
};

enum Automata {
    Character {
        set: BTreeSet<char>,
        accept: Accept,
    },
    Repetition {
        body: Box<Automata>,
        min: Option<usize>,
        max: Option<usize>,
    },
    Selection(Vec<Automata>),
    Sequence(Vec<Automata>),
}

enum Accept {
    Complement,
    Set,
}
