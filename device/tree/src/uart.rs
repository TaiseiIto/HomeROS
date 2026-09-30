use crate::Node;

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
        let base_address: usize = node
            .regions()
            .min()
            .map(|base_address| base_address as usize)
            .ok_or(())?;
        let frequency_hz: u128 = node
            .clock_frequency()
            .map(|clock_frequency| clock_frequency as u128)
            .or(node.name2clock().get("uartclk").copied())
            .ok_or(())?;
        let standard: Standard = node
            .compatibles()
            .into_iter()
            .find_map(|compatible| match compatible.device() {
                "pl011" => Some(Standard::Pl011),
                "ns16550a" => Some(Standard::Ns16550a),
                _ => None,
            })
            .ok_or(())?;
        Ok(Self {
            base_address,
            frequency_hz,
            standard,
        })
    }
}

#[derive(Clone, Debug)]
pub enum Standard {
    Pl011,
    Ns16550a,
}
