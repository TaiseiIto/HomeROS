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
pub enum Specifiers {
    Raw(Vec<u32>),
    Pretty(Vec<u128>),
}

impl Debug for Specifiers {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::Raw(words) => formatter.debug_list().entries(words).finish(),
            Self::Pretty(specifiers) => formatter.debug_list().entries(specifiers).finish(),
        }
    }
}

impl SecondAnalyzed for Specifiers {
    fn second_analyze(&self, second_analyzer: &SecondAnalyzer<'_>) -> Self {
        if let Self::Raw(words) = self {
            Self::Pretty(
                words
                    .as_slice()
                    .chunks(second_analyzer.interrupt_parent_interrupt_cells())
                    .map(|specifier| {
                        specifier
                            .iter()
                            .fold(0, |value, cell| (value << u32::BITS) + (*cell as u128))
                    })
                    .collect(),
            )
        } else {
            panic!();
        }
    }
}
