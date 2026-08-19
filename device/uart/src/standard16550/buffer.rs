/// # References
/// * [Table of Registers](https://www.lookrs232.com/rs232/registers.htm)
#[io::register]
pub struct Register {
    data: u8,
}

impl Register {
    pub fn send(data: u8) -> Self {
        Self::default().update_data_u8(data)
    }
}
