use {
    crate::{Analyzed, Node},
    core::ops::Range,
};

#[derive(Debug)]
pub struct Information {
    // frequency_hz: u128,
    // memory_region: Range<u128>,
    standard: Standard,
}

impl TryFrom<&Node> for Information {
    type Error = ();

    fn try_from(node: &Node) -> Result<Self, Self::Error> {
        node.children()
            .iter()
            .find_map(|child| child.try_into().ok())
            .or({
                let standard: Option<Standard> =
                    node.compatibles().into_iter().find_map(|compatible| {
                        match compatible.device() {
                            "pl011" => Some(Standard::Pl011),
                            "ns16550a" => Some(Standard::Ns16550a),
                            _ => None,
                        }
                    });
                match standard {
                    Some(standard) => Some(Self {
                        // frequency_hz,
                        // memory_region,
                        standard,
                    }),
                    _ => None,
                }
            })
            .ok_or(())
    }
}

#[derive(Debug)]
pub enum Standard {
    Pl011,
    Ns16550a,
}
