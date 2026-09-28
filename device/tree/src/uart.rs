use {
    crate::{Analyzed, Node},
    core::ops::Range,
};

#[derive(Debug)]
pub struct Information {
    // frequency_hz: u128,
    base_address: usize,
    standard: Standard,
}

impl TryFrom<&Node> for Information {
    type Error = ();

    fn try_from(node: &Node) -> Result<Self, Self::Error> {
        let base_address: Option<usize> = node
            .regions()
            .min()
            .map(|base_address| base_address as usize);
        let standard: Option<Standard> =
            node.compatibles()
                .into_iter()
                .find_map(|compatible| match compatible.device() {
                    "pl011" => Some(Standard::Pl011),
                    "ns16550a" => Some(Standard::Ns16550a),
                    _ => None,
                });
        base_address
            .zip(standard)
            .map(|(base_address, standard)| Self {
                // frequency_hz,
                base_address,
                standard,
            })
            .ok_or(())
    }
}

#[derive(Debug)]
pub enum Standard {
    Pl011,
    Ns16550a,
}
