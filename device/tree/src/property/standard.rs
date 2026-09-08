use {
    alloc::{string::String, vec::Vec},
    core::str::Split,
};

/// # References
/// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 3.6 /chosen Node
#[derive(Clone, Debug)]
pub struct Path {
    path: Vec<String>,
    unit_address: Option<u128>,
}

impl From<&str> for Path {
    fn from(string: &str) -> Self {
        let mut string: Split<'_, char> = string.split('@');
        Self {
            path: string
                .next()
                .unwrap()
                .split('/')
                .map(String::from)
                .collect(),
            unit_address: string
                .next()
                .map(|unit_address| u128::from_str_radix(unit_address, 16).unwrap()),
        }
    }
}
