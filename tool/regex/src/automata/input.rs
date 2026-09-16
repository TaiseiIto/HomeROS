use alloc::collections::vec_deque::VecDeque;

#[derive(Clone)]
pub struct SearchPoint(VecDeque<Character>);

impl SearchPoint {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<&str> for SearchPoint {
    fn from(string: &str) -> Self {
        Self(
            string
                .char_indices()
                .map(|(index, character)| Character { character, index })
                .collect(),
        )
    }
}

impl Iterator for SearchPoint {
    type Item = Character;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
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
