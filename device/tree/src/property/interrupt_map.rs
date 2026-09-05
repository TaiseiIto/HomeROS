use {
    crate::node::{SecondAnalyzed, SecondAnalyzer},
    alloc::vec::Vec,
    core::{
        fmt::{Debug, Formatter, Result},
        slice::Iter,
    },
};

/// # References
/// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.4.3 Interrupt Nexus Properties
#[derive(Clone)]
pub enum InterruptMap {
    Raw(Vec<u32>),
    Pretty(Vec<Entry>),
}

impl Debug for InterruptMap {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::Raw(words) => formatter.debug_list().entries(words).finish(),
            Self::Pretty(entries) => formatter.debug_list().entries(entries).finish(),
        }
    }
}

impl SecondAnalyzed for InterruptMap {
    fn second_analyze(&self, second_analyzer: &SecondAnalyzer<'_>) -> Self {
        if let Self::Raw(words) = self {
            let address_cells: usize = second_analyzer.address_cells();
            let interrupt_cells: usize = second_analyzer.interrupt_cells();
            let mut words: Iter<'_, u32> = words.iter();
            let mut entries: Vec<Entry> = Vec::new();
            while words.len() != 0 {
                let child_unit_address: u128 = (0..address_cells)
                    .map(|_| words.next().unwrap())
                    .fold(0, |value, cell| (value << u32::BITS) + (*cell as u128));
                let child_interrupt_specifier: u128 = (0..interrupt_cells)
                    .map(|_| words.next().unwrap())
                    .fold(0, |value, cell| (value << u32::BITS) + (*cell as u128));
                let interrupt_parent: u32 = *words.next().unwrap();
                let interrupt_parent_address_cells: usize =
                    second_analyzer.phandle_address_cells(interrupt_parent);
                let interrupt_parent_interrupt_cells: usize =
                    second_analyzer.phandle_interrupt_cells(interrupt_parent);
                let parent_unit_address: u128 = (0..interrupt_parent_address_cells)
                    .map(|_| words.next().unwrap())
                    .fold(0, |value, cell| (value << u32::BITS) + (*cell as u128));
                let parent_interrupt_specifier: u128 = (0..interrupt_parent_interrupt_cells)
                    .map(|_| words.next().unwrap())
                    .fold(0, |value, cell| (value << u32::BITS) + (*cell as u128));
                entries.push(Entry {
                    child_unit_address,
                    child_interrupt_specifier,
                    interrupt_parent,
                    parent_unit_address,
                    parent_interrupt_specifier,
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
    child_unit_address: u128,
    child_interrupt_specifier: u128,
    interrupt_parent: u32,
    parent_unit_address: u128,
    parent_interrupt_specifier: u128,
}
