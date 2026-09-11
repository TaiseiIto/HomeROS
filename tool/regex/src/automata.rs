use {super::symbol, alloc::collections::btree_set::BTreeSet};

struct Character {
    set: BTreeSet<char>,
    accept: Accept,
}

impl Character {
    fn accept(&self, character: &char) -> bool {
        let Self { set, accept } = self;
        match accept {
            Accept::Complement => !set.contains(character),
            Accept::Set => set.contains(character),
        }
    }
}

enum Accept {
    Complement,
    Set,
}
