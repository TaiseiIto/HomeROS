use {
    crate::node::{SecondAnalyzed, SecondAnalyzer},
    alloc::vec::Vec,
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.3.8 ranges
#[derive(Clone)]
pub enum Ranges {
    Raw(Vec<u32>),
    Pretty(Vec<Range>),
}

impl Debug for Ranges {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::Raw(words) => formatter.debug_list().entries(words).finish(),
            Self::Pretty(ranges) => formatter.debug_list().entries(ranges).finish(),
        }
    }
}

impl SecondAnalyzed for Ranges {
    fn second_analyze(&self, second_analyzer: &SecondAnalyzer<'_>) -> Self {
        if let Self::Raw(words) = self {
            let child_bus_address_cells: usize = second_analyzer.address_cells();
            let parent_bus_address_cells: usize = second_analyzer.parent_address_cells();
            let size_cells: usize = second_analyzer.size_cells();
            let range_cells: usize =
                child_bus_address_cells + parent_bus_address_cells + size_cells;
            Self::Pretty(
                words
                    .as_slice()
                    .chunks(range_cells)
                    .map(|range| {
                        let child_bus_address_start: usize = 0;
                        let child_bus_address_end: usize =
                            child_bus_address_start + child_bus_address_cells;
                        let child_bus_address: u128 = range
                            [child_bus_address_start..child_bus_address_end]
                            .iter()
                            .fold(0, |value, cell| (value << u32::BITS) + (*cell as u128));
                        let parent_bus_address_start: usize = child_bus_address_end;
                        let parent_bus_address_end: usize =
                            parent_bus_address_start + parent_bus_address_cells;
                        let parent_bus_address: u128 = range
                            [parent_bus_address_start..parent_bus_address_end]
                            .iter()
                            .fold(0, |value, cell| (value << u32::BITS) + (*cell as u128));
                        let length_start: usize = parent_bus_address_end;
                        let length_end: usize = length_start + size_cells;
                        let length: u128 = range[length_start..length_end]
                            .iter()
                            .fold(0, |value, cell| (value << u32::BITS) + (*cell as u128));
                        Range {
                            child_bus_address,
                            parent_bus_address,
                            length,
                        }
                    })
                    .collect(),
            )
        } else {
            panic!();
        }
    }
}

#[derive(Clone, Debug)]
struct Range {
    child_bus_address: u128,
    parent_bus_address: u128,
    length: u128,
}
