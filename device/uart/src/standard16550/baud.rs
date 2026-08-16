/// # References
/// * [Table of Registers](https://www.lookrs232.com/rs232/registers.htm)
#[io::registers]
struct Divisor {
    low: Low,
    high: High,
}

/// # References
/// * [Table of Registers](https://www.lookrs232.com/rs232/registers.htm)
#[io::register]
struct Low {
    byte: u8,
}

/// # References
/// * [Table of Registers](https://www.lookrs232.com/rs232/registers.htm)
#[io::register]
struct High {
    byte: u8,
}
