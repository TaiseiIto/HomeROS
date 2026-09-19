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
        alloc::{collections::btree_map::BTreeMap, string::String, vec::Vec},
    };

    #[test]
    fn test() {
        let automaton: Automaton = r"^(clk(?<value>\d+)(?<unit>[kmgtpxzyrq]?)hz,?){2}$"
            .parse()
            .unwrap();
        let matches: Vec<Match> = automaton.input("clk24mhz,clk115200hz");
        let captures: Vec<BTreeMap<String, Vec<Capture>>> =
            matches.iter().map(|mat| mat.captures()).collect();
    }
}
