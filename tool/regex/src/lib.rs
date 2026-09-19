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
        alloc::{
            collections::btree_map::BTreeMap,
            string::{String, ToString},
            vec,
            vec::Vec,
        },
    };

    #[test]
    fn test() {
        let automaton: Automaton = r"^(clk(?<value>\d+)(?<unit>[kmgtpxzyrq]?)hz,?){2}$"
            .parse()
            .unwrap();
        let matches: Vec<Match> = automaton.input("clk24mhz,clk115200hz");
        let captures: Vec<BTreeMap<String, Vec<String>>> = matches
            .iter()
            .map(|mat| {
                mat.captures()
                    .into_iter()
                    .map(|(name, captures)| {
                        (
                            name,
                            captures
                                .into_iter()
                                .map(|capture| capture.to_string())
                                .collect(),
                        )
                    })
                    .collect()
            })
            .collect();
        let expected: Vec<BTreeMap<String, Vec<String>>> = vec![BTreeMap::from([
            (
                "".to_string(),
                vec![
                    "clk24mhz,clk115200hz".to_string(),
                    "clk24mhz,".to_string(),
                    "clk115200hz".to_string(),
                ],
            ),
            ("unit".to_string(), vec!["m".to_string()]),
            (
                "value".to_string(),
                vec!["24".to_string(), "115200".to_string()],
            ),
        ])];
        assert_eq!(captures, expected);
    }
}
