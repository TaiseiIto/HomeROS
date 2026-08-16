/// # References
/// * [Table of Registers](https://www.lookrs232.com/rs232/registers.htm)
#[io::register]
pub struct Low {
    byte: u8,
}

/// # References
/// * [Table of Registers](https://www.lookrs232.com/rs232/registers.htm)
#[io::register]
pub struct High {
    byte: u8,
}
