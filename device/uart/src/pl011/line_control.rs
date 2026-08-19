use super::super::Parity;

/// # References
/// * [Line Control Register](https://support.arm.com/documentation/ddi0183/g/programmers-model/register-descriptions/line-control-register--uartlcr-h?lang=en)
#[io::register]
pub struct Register {
    send_break: bool,
    parity_enable: bool,
    even_parity: bool,
    two_stop_bits: bool,
    enable_fifo: bool,
    word_length: [bool; 2],
    stick_parity: bool,
    __: [u8; 3],
}

impl Register {
    pub fn new(
        enable_fifo: bool,
        parity: Option<Parity>,
        send_break: bool,
        stop_bit: u8,
        word_length: u8,
    ) -> Self {
        Self::default()
            .update_send_break_bit(send_break)
            .update_two_stop_bits_bit(match stop_bit {
                1 => false,
                2 => true,
                _ => panic!(),
            })
            .update_enable_fifo_bit(enable_fifo)
            .update_word_length_shift(match word_length {
                5 => 0,
                6 => 1,
                7 => 2,
                8 => 3,
                _ => panic!(),
            })
            .update_parity(parity)
    }

    fn update_parity(self, parity: Option<Parity>) -> Self {
        if let Some(parity) = parity {
            self.update_parity_enable_bit(true)
                .update_even_parity_bit(match parity {
                    Parity::High | Parity::Odd => false,
                    Parity::Even | Parity::Low => true,
                })
                .update_stick_parity_bit(match parity {
                    Parity::Even | Parity::Odd => false,
                    Parity::High | Parity::Low => true,
                })
        } else {
            self.update_parity_enable_bit(false)
        }
    }
}
