/// # References
/// * [Line Control Register](https://www.lookrs232.com/rs232/lcr.htm)
#[io::field]
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
        match self.word_length_bits_read() {
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
            .then(|| match control.parity_type_bits_read() {
                [false, false] => Parity::Odd,
                [true, false] => Parity::Even,
                [false, true] => Parity::High,
                [true, true] => Parity::Low,
            })
    }
}

/// # References
/// * [Line Status Register](https://www.lookrs232.com/rs232/lsr.htm)
#[io::field]
struct Status {
    data_ready: bool,
    overrun_error: bool,
    parity_error: bool,
    framing_error: bool,
    break_interrupt: bool,
    empty_transmitter: bool,
    empty_data: bool,
    received_fifo_error: bool,
}
