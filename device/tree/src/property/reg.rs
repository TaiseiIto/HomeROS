use {
    crate::node::{SecondAnalyzed, SecondAnalyzer},
    alloc::vec::Vec,
    core::{
        fmt::{Debug, Formatter, Result},
        ops::Range,
    },
};

/// # References
/// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.3.6 reg
#[derive(Clone)]
pub enum Reg {
    Raw(Vec<u32>),
    Pretty(Vec<Range<usize>>),
}

impl Debug for Reg {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::Raw(words) => formatter.debug_list().entries(words).finish(),
            Self::Pretty(ranges) => formatter.debug_list().entries(ranges).finish(),
        }
    }
}

impl SecondAnalyzed for Reg {
    fn second_analyze(&self, second_analyzer: &SecondAnalyzer<'_>) -> Self {
        if let Self::Raw(words) = self {
            let address_cells: usize = second_analyzer.parent_address_cells();
            let size_cells: usize = second_analyzer.parent_size_cells();
            let pair_cells: usize = address_cells + size_cells;
            Self::Pretty(
                words
                    .as_slice()
                    .chunks(pair_cells)
                    .map(|pair| {
                        let (address_cells, size_cells): (&[u32], &[u32]) =
                            pair.split_at(address_cells);
                        let start: usize = address_cells
                            .iter()
                            .rev()
                            .fold(0, |value, cell| (value << u32::BITS) + (*cell as usize));
                        let size: usize = size_cells
                            .iter()
                            .rev()
                            .fold(0, |value, cell| (value << u32::BITS) + (*cell as usize));
                        let end: usize = start + size;
                        (start..end)
                    })
                    .collect(),
            )
        } else {
            panic!();
        }
    }
}
