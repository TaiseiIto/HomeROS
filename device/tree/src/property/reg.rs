use {
    alloc::vec::Vec,
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.3.6 reg
pub enum Reg {
    Raw(Vec<u32>),
    Pretty(Vec<Pair>),
}

impl Debug for Reg {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::Raw(words) => formatter.debug_list().entries(words).finish(),
            Self::Pretty(pairs) => formatter.debug_list().entries(pairs).finish(),
        }
    }
}

#[derive(Debug)]
struct Pair {
    address: usize,
    length: usize,
}
