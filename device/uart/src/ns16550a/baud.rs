/// # References
/// * [Table of Registers](https://www.lookrs232.com/rs232/registers.htm)
#[io::register]
pub struct Low {
    byte: u8,
}

impl Low {
    pub fn new(low: u8) -> Self {
        Self::default().update_byte_u8(low)
    }
}

/// # References
/// * [Table of Registers](https://www.lookrs232.com/rs232/registers.htm)
#[io::register]
pub struct High {
    byte: u8,
}

impl High {
    pub fn new(high: u8) -> Self {
        Self::default().update_byte_u8(high)
    }
}
