/// # References
/// * [Line Control Register](https://www.lookrs232.com/rs232/lcr.htm)
#[bit::field]
struct Control {
    word_length: [bool; 2],
    stop_bit_length: bool,
    parity_enable: bool,
    parity_type: [bool; 2],
    set_break: bool,
    divisor_latch_access: bool,
}

impl Control {
    fn word_length(&self) -> u8 {
        match self.word_length_bit_read() {
            [false, false] => 5,
            [true, false] => 6,
            [false, true] => 7,
            [true, true] => 8,
        }
    }
}

enum Parity {
    Odd,
    Even,
    High,
    Low,
}

impl From<&Control> for Option<Parity> {
    fn from(control: &Control) -> Self {
        control
            .parity_enable_bit_read()
            .then(|| match control.parity_type_bit_read() {
                [false, false] => Parity::Odd,
                [true, false] => Parity::Even,
                [false, true] => Parity::High,
                [true, true] => Parity::Low,
            })
    }
}
