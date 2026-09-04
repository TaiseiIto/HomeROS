use alloc::vec::Vec;

/// # References
/// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.3.6 reg
#[derive(Debug)]
pub enum Reg {
    Raw(Vec<u32>),
    Pretty(Vec<Pair>),
}

#[derive(Debug)]
struct Pair {
    address: usize,
    length: usize,
}
