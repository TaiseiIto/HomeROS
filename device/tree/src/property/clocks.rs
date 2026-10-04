use {
    crate::node::{SecondAnalyzed, SecondAnalyzer},
    alloc::{collections::btree_map::BTreeMap, string::String, vec::Vec},
    core::fmt::{Debug, Formatter, Result},
    regex::{Automaton, Capture, Match},
    unit::prefix::{EXA, GIGA, KILO, MEGA, PETA, QUETTA, RONNA, TERA, YOTTA, ZETTA},
};

#[derive(Clone)]
pub enum Clocks {
    Raw(Vec<u32>),
    Pretty(Vec<u128>),
}

impl Debug for Clocks {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::Raw(words) => formatter.debug_list().entries(words).finish(),
            Self::Pretty(clocks) => formatter.debug_list().entries(clocks).finish(),
        }
    }
}

impl SecondAnalyzed for Clocks {
    fn second_analyze(&self, second_analyzer: &SecondAnalyzer<'_>) -> Self {
        if let Self::Raw(phandles) = self {
            Self::Pretty(
                phandles
                    .iter()
                    .flat_map(|phandle| {
                        second_analyzer
                            .phandle_names(*phandle, "clock-output")
                            .into_iter()
                            .map(|clock| {
                                let automaton: Automaton =
                                    r"^clk(?<value>\d+)(?<unit_prefix>[kmgtpxzyrq]?)hz$"
                                        .parse()
                                        .unwrap();
                                let mat: Match =
                                    automaton.input(clock.as_str()).into_iter().next().unwrap();
                                let captures: BTreeMap<String, Vec<Capture>> = mat.captures();
                                let value: &str = captures["value"].first().unwrap().into();
                                let value: u128 = value.parse().unwrap();
                                let unit_prefix: u128 = match captures
                                    .get("unit_prefix")
                                    .map(|captures| captures.iter().next().unwrap().into())
                                {
                                    None => 1,
                                    Some("k") => KILO,
                                    Some("m") => MEGA,
                                    Some("g") => GIGA,
                                    Some("t") => TERA,
                                    Some("p") => PETA,
                                    Some("x") => EXA,
                                    Some("z") => ZETTA,
                                    Some("y") => YOTTA,
                                    Some("r") => RONNA,
                                    Some("q") => QUETTA,
                                    _ => panic!(),
                                };
                                unit_prefix * value
                            })
                    })
                    .collect(),
            )
        } else {
            panic!();
        }
    }
}
