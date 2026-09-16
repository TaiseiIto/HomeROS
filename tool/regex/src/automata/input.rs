use alloc::collections::vec_deque::VecDeque;

#[derive(Clone)]
pub struct SearchPoint(VecDeque<Character>);

impl SearchPoint {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn pop_front(&mut self) -> Option<Character> {
        self.0.pop_front()
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
