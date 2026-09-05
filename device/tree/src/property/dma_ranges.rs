use {
    crate::node::{SecondAnalyzed, SecondAnalyzer},
    alloc::vec::Vec,
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.3.9 dma-ranges
#[derive(Clone)]
pub enum DmaRanges {
    Raw(Vec<u32>),
    Pretty(Vec<DmaRange>),
}

impl Debug for DmaRanges {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::Raw(words) => formatter.debug_list().entries(words).finish(),
            Self::Pretty(dma_ranges) => formatter.debug_list().entries(dma_ranges).finish(),
        }
    }
}

impl SecondAnalyzed for DmaRanges {
    fn second_analyze(&self, second_analyzer: &SecondAnalyzer<'_>) -> Self {
        if let Self::Raw(words) = self {
            let child_bus_address_cells: usize = second_analyzer.address_cells();
            let parent_bus_address_cells: usize = second_analyzer.parent_address_cells();
            let size_cells: usize = second_analyzer.size_cells();
            let dma_range_cells: usize =
                child_bus_address_cells + parent_bus_address_cells + size_cells;
            Self::Pretty(
                words
                    .as_slice()
                    .chunks(dma_range_cells)
                    .map(|dma_range| {
                        let child_bus_address_start: usize = 0;
                        let child_bus_address_end: usize =
                            child_bus_address_start + child_bus_address_cells;
                        let child_bus_address: usize = dma_range
                            [child_bus_address_start..child_bus_address_end]
                            .iter()
                            .fold(0, |value, cell| (value << u32::BITS) + (*cell as usize));
                        let parent_bus_address_start: usize = child_bus_address_end;
                        let parent_bus_address_end: usize =
                            parent_bus_address_start + parent_bus_address_cells;
                        let parent_bus_address: usize = dma_range
                            [parent_bus_address_start..parent_bus_address_end]
                            .iter()
                            .fold(0, |value, cell| (value << u32::BITS) + (*cell as usize));
                        let length_start: usize = parent_bus_address_end;
                        let length_end: usize = length_start + size_cells;
                        let length: usize = dma_range[length_start..length_end]
                            .iter()
                            .fold(0, |value, cell| (value << u32::BITS) + (*cell as usize));
                        DmaRange {
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
struct DmaRange {
    child_bus_address: usize,
    parent_bus_address: usize,
    length: usize,
}
