use {
    crate::node::{SecondAnalyzedWithSpecifier, SecondAnalyzer},
    alloc::vec::Vec,
    core::{
        fmt::{Debug, Formatter, Result},
        slice::Iter,
    },
};

/// # References
/// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.5.1 Nexus Node Properties
#[derive(Clone)]
pub enum Map {
    Raw(Vec<u32>),
    Pretty(Vec<Entry>),
}

impl Debug for Map {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::Raw(words) => formatter.debug_list().entries(words).finish(),
            Self::Pretty(entries) => formatter.debug_list().entries(entries).finish(),
        }
    }
}

impl SecondAnalyzedWithSpecifier for Map {
    fn second_analyze_with_specifier(
        &self,
        second_analyzer: &SecondAnalyzer<'_>,
        specifier: &str,
    ) -> Self {
        if let Self::Raw(words) = self {
            let specifier_cells: usize = second_analyzer.specifier_cells(specifier).unwrap();
            let mut words: Iter<'_, u32> = words.iter();
            let mut entries: Vec<Entry> = Vec::new();
            while words.len() != 0 {
                let child_specifier: u128 = (0..specifier_cells)
                    .map(|_| words.next().unwrap())
                    .fold(0, |value, cell| (value << u32::BITS) + (*cell as u128));
                let specifier_parent: u32 = *words.next().unwrap();
                let specifier_parent_cells: usize = second_analyzer
                    .phandle_specifier_cells(specifier_parent, specifier)
                    .unwrap();
                let parent_specifier: u128 = (0..specifier_parent_cells)
                    .map(|_| words.next().unwrap())
                    .fold(0, |value, cell| (value << u32::BITS) + (*cell as u128));
                entries.push(Entry {
                    child_specifier,
                    specifier_parent,
                    parent_specifier,
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
    child_specifier: u128,
    specifier_parent: u32,
    parent_specifier: u128,
}
