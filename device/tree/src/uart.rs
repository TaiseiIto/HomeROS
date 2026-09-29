use {
    crate::{Analyzed, Node},
    core::ops::Range,
};

#[derive(Debug)]
pub struct Uart {
    base_address: usize,
    frequency_hz: u128,
    standard: Standard,
}

impl Uart {
    pub fn base_address(&self) -> usize {
        self.base_address
    }

    pub fn frequency_hz(&self) -> u128 {
        self.frequency_hz
    }

    pub fn standard(&self) -> Standard {
        self.standard.clone()
    }
}

impl TryFrom<&Node> for Uart {
    type Error = ();

    fn try_from(node: &Node) -> Result<Self, Self::Error> {
        let base_address: Option<usize> = node
            .regions()
            .min()
            .map(|base_address| base_address as usize);
        let frequency_hz: Option<u128> = node
            .clock_frequency()
            .map(|clock_frequency| clock_frequency as u128)
            .or(node.name2clock().get("uartclk").copied());
        let standard: Option<Standard> =
            node.compatibles()
                .into_iter()
                .find_map(|compatible| match compatible.device() {
                    "pl011" => Some(Standard::Pl011),
                    "ns16550a" => Some(Standard::Ns16550a),
                    _ => None,
                });
        match (base_address, frequency_hz, standard) {
            (Some(base_address), Some(frequency_hz), Some(standard)) => Ok(Self {
                base_address,
                frequency_hz,
                standard,
            }),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Standard {
    Pl011,
    Ns16550a,
}
