#![no_std]

extern crate alloc;

mod automaton;
mod character;
pub mod search;
mod symbol;

pub use {
    automaton::Automaton,
    search::result::{Capture, Match},
};

#[cfg(test)]
mod test {
    use {
        super::*,
        alloc::{vec, vec::Vec},
    };

    #[test]
    fn test() {
        let automaton: Automaton = r"^clk(\d+)([kmgtpxzyrq])hz$".parse().unwrap();
        let matches: Vec<Match> = automaton.input("clk24mhz");
        let captures: Vec<Capture> = matches.iter().flat_map(|mat| mat.captures()).collect();
        let captures: Vec<&str> = captures.iter().map(Into::into).collect();
        assert_eq!(captures, vec!["clk24mhz", "24", "m"]);
    }
}
