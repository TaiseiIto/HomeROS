/// # References
/// * [Line Control Register](https://www.lookrs232.com/rs232/lcr.htm)
#[io::register]
pub struct Control {
    word_length: [bool; 2],
    stop_bit_length: bool,
    parity_enable: bool,
    parity_type: [bool; 2],
    set_break: bool,
    divisor_latch_access: bool,
}

/// # References
/// * [Line Status Register](https://www.lookrs232.com/rs232/lsr.htm)
#[io::register]
pub struct Status {
    data_ready: bool,
    overrun_error: bool,
    parity_error: bool,
    framing_error: bool,
    break_interrupt: bool,
    empty_transmitter: bool,
    empty_data: bool,
    received_fifo_error: bool,
}
