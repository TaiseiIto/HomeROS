use {
    crate::node::{SecondAnalyzed, SecondAnalyzer},
    alloc::vec::Vec,
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 3.5.2 /reserved-memory/ child nodes
#[derive(Clone)]
pub enum Alignment {
    Raw(Vec<u32>),
    Pretty(usize),
}

impl Debug for Alignment {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::Raw(words) => formatter.debug_list().entries(words).finish(),
            Self::Pretty(alignment) => formatter.debug_tuple("Alignment").field(alignment).finish(),
        }
    }
}

impl SecondAnalyzed for Alignment {
    fn second_analyze(&self, second_analyzer: &SecondAnalyzer<'_>) -> Self {
        if let Self::Raw(words) = self {
            Self::Pretty(
                words[0..second_analyzer.parent_size_cells()]
                    .iter()
                    .fold(0, |value, cell| (value << u32::BITS) + (*cell as usize)),
            )
        } else {
            panic!();
        }
    }
}
