use super::super::Parity;

/// # References
/// * [Line Control Register](https://www.lookrs232.com/rs232/lcr.htm)
#[io::register]
pub struct Control {
    word_length: [bool; 2],
    stop_bit_length: bool,
    parity_enable: bool,
    parity_type: [bool; 2],
    send_break: bool,
    divisor_latch_access: bool,
}

impl Control {
    pub fn set(parity: Option<Parity>, send_break: bool, stop_bit: u8, word_length: u8) -> Self {
        let value: Self = Self::default()
            .update_word_length_shift(match word_length {
                5 => 0,
                6 => 1,
                7 => 2,
                8 => 3,
                _ => panic!(),
            })
            .update_stop_bit_length_bit(match stop_bit {
                1 => false,
                2 => true,
                _ => panic!(),
            })
            .update_send_break_bit(send_break);
        if let Some(parity) = parity {
            value
                .update_parity_enable_bit(true)
                .update_parity_type_shift(match parity {
                    Parity::Even => 1,
                    Parity::High => 2,
                    Parity::Low => 3,
                    Parity::Odd => 0,
                })
        } else {
            value.update_parity_enable_bit(false)
        }
    }
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
