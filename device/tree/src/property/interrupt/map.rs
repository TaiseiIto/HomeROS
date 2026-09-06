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
pub enum Mask {
    Raw(Vec<u32>),
    Pretty {
        child_unit_address: u128,
        child_interrupt_specifier: u128,
    },
}

impl Debug for Mask {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::Raw(words) => formatter.debug_list().entries(words).finish(),
            Self::Pretty {
                child_unit_address,
                child_interrupt_specifier,
            } => formatter
                .debug_struct("Mask")
                .field("child_unit_address", child_unit_address)
                .field("child_interrupt_specifier", child_interrupt_specifier)
                .finish(),
        }
    }
}

impl SecondAnalyzed for Mask {
    fn second_analyze(&self, second_analyzer: &SecondAnalyzer<'_>) -> Self {
        if let Self::Raw(words) = self {
            let (child_unit_address, child_interrupt_specifier): (&[u32], &[u32]) =
                words.as_slice().split_at(second_analyzer.address_cells());
            assert_eq!(
                child_interrupt_specifier.len(),
                second_analyzer.interrupt_cells()
            );
            Self::Pretty {
                child_unit_address: child_unit_address
                    .iter()
                    .fold(0, |value, cell| (value << u32::BITS) + (*cell as u128)),
                child_interrupt_specifier: child_interrupt_specifier
                    .iter()
                    .fold(0, |value, cell| (value << u32::BITS) + (*cell as u128)),
            }
        } else {
            panic!();
        }
    }
}
