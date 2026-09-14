use alloc::collections::vec_deque::VecDeque;

#[derive(Clone)]
pub struct String(VecDeque<Character>);

impl String {
    pub fn pop_front(&mut self) -> Option<Character> {
        self.0.pop_front()
    }
}

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

#[derive(Clone)]
pub struct Character {
    character: char,
    index: usize,
}

impl Character {
    pub fn character(&self) -> char {
        self.character
    }

    pub fn index(&self) -> usize {
        self.index
    }
}
