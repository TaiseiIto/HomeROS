use {
    crate::node::{SecondAnalyzed, SecondAnalyzer},
    alloc::{string::String, vec::Vec},
    core::fmt::{Debug, Formatter, Result},
};

#[derive(Clone)]
pub enum Clocks {
    Raw(Vec<u32>),
    Pretty(Vec<Vec<String>>),
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
                    .map(|phandle| {
                        second_analyzer
                            .phandle_names(*phandle, "clock-output")
                            .unwrap()
                    })
                    .collect(),
            )
        } else {
            panic!();
        }
    }
}
