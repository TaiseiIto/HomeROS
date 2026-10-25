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
            format,
            string::{String, ToString},
            vec,
            vec::Vec,
        },
    };

    #[test]
    fn device_tree() {
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

    #[test]
    fn guid() {
        let automaton: Automaton = r"^(?<data1>[\da-f]{8})-(?<data2>[\da-f]{4})-(?<data3>[\da-f]{4})-(?<data4prefix>[\da-f]{4})-(?<data4suffix>[\da-f]{12})$".parse().unwrap();
        let matches: Vec<Match> = automaton.input("8868e871-e4f1-11d3-bc22-0080c73c8881");
        let mat: Match = matches.into_iter().next().ok_or(()).unwrap();
        let captures: BTreeMap<String, Vec<Capture>> = mat.captures();
        let _data1: u32 = u32::from_str_radix(&captures["data1"].first().unwrap().to_string(), 16)
            .map_err(|_| ())
            .unwrap();
        let _data2: u16 = u16::from_str_radix(&captures["data2"].first().unwrap().to_string(), 16)
            .map_err(|_| ())
            .unwrap();
        let _data3: u16 = u16::from_str_radix(&captures["data3"].first().unwrap().to_string(), 16)
            .map_err(|_| ())
            .unwrap();
        let data4prefix: Vec<char> = captures["data4prefix"]
            .first()
            .unwrap()
            .to_string()
            .as_str()
            .chars()
            .collect();
        let data4prefix: Vec<u8> = data4prefix
            .chunks(2)
            .map(|byte| u8::from_str_radix(&format!("{}{}", byte[0], byte[1]), 16).unwrap())
            .collect();
        let data4prefix: [u8; 2] = data4prefix.try_into().map_err(|_| ()).unwrap();
        let data4suffix: Vec<char> = captures["data4suffix"]
            .first()
            .unwrap()
            .to_string()
            .as_str()
            .chars()
            .collect();
        let data4suffix: Vec<u8> = data4suffix
            .chunks(2)
            .map(|byte| u8::from_str_radix(&format!("{}{}", byte[0], byte[1]), 16).unwrap())
            .collect();
        let data4suffix: [u8; 6] = data4suffix.try_into().map_err(|_| ()).unwrap();
        let _data4: Vec<u8> = data4prefix.into_iter().chain(data4suffix).collect();
    }

    #[test]
    fn start_of_line() {
        let automaton: Automaton = r"bc".parse().unwrap();
        assert!(!automaton.input("abc").is_empty());
        let automaton: Automaton = r"^bc".parse().unwrap();
        assert!(automaton.input("abc").is_empty());
    }
}
