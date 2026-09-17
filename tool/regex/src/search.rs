pub mod result;

use alloc::collections::vec_deque::VecDeque;

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

#[derive(Clone)]
pub struct Point(VecDeque<Character>);

impl Point {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<&str> for Point {
    fn from(string: &str) -> Self {
        Self(
            string
                .char_indices()
                .map(|(index, character)| Character { character, index })
                .collect(),
        )
    }
}

impl Iterator for Point {
    type Item = Character;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

#[derive(Clone)]
pub struct Target(Point);

impl From<&str> for Target {
    fn from(string: &str) -> Self {
        Self(string.into())
    }
}

impl Iterator for Target {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let next: Point = self.0.clone();
        self.0.next().is_some().then_some(next)
    }
}
