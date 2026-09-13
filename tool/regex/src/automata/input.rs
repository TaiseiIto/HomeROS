use alloc::vec::Vec;

pub struct String(Vec<Character>);

impl From<&str> for String {
    fn from(string: &str) -> Self {
        Self(
            string
                .char_indices()
                .map(|(index, character)| Character { character, index })
                .collect(),
        )
    }
}

struct Character {
    character: char,
    index: usize,
}
