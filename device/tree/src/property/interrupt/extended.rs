use {
    crate::node::{SecondAnalyzed, SecondAnalyzer},
    alloc::vec::Vec,
    core::{
        fmt::{Debug, Formatter, Result},
        slice::Iter,
    },
};

/// # References
/// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.4.1 Properties for Interrupt Generating Devices
#[derive(Clone)]
pub enum Extended {
    Raw(Vec<u32>),
    Pretty(Vec<Entry>),
}

impl Debug for Extended {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::Raw(words) => formatter.debug_list().entries(words).finish(),
            Self::Pretty(entries) => formatter.debug_list().entries(entries).finish(),
        }
    }
}

impl SecondAnalyzed for Extended {
    fn second_analyze(&self, second_analyzer: &SecondAnalyzer<'_>) -> Self {
        if let Self::Raw(words) = self {
            let mut words: Iter<'_, u32> = words.iter();
            let mut entries: Vec<Entry> = Vec::new();
            while let Some(phandle) = words.next().copied() {
                entries.push(Entry {
                    phandle,
                    specifier: (0..second_analyzer.phandle_interrupt_cells(phandle).unwrap())
                        .map(|_| words.next().unwrap())
                        .fold(0, |value, cell| (value << u32::BITS) + (*cell as u128)),
                });
            }
            Self::Pretty(entries)
        } else {
            panic!();
        }
    }
}

#[derive(Clone, Debug)]
struct Entry {
    phandle: u32,
    specifier: u128,
}
