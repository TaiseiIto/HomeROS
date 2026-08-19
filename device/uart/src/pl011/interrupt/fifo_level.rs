/// # References
/// * [Interrupt FIFO Level Select Register](https://support.arm.com/documentation/ddi0183/g/programmers-model/register-descriptions/interrupt-fifo-level-select-register--uartifls?lang=en)
#[io::register]
pub struct Register {
    transmit: [bool; 3],
    receive: [bool; 3],
    __: [bool; 26],
}

impl Register {
    pub fn new(transmit_ratio_8times: u8, receive_ratio_8times: u8) -> Self {
        Self::default()
            .update_transmit_shift(Self::ratio_8times_to_shift(transmit_ratio_8times))
            .update_receive_shift(Self::ratio_8times_to_shift(receive_ratio_8times))
    }

    fn ratio_8times_to_shift(ratio_8times: u8) -> u32 {
        match ratio_8times {
            1 => 0, // Interrupt when 1/8 of FIFO is full
            2 => 1, // Interrupt when 1/4 of FIFO is full
            4 => 2, // Interrupt when 1/2 of FIFO is full
            6 => 3, // Interrupt when 3/4 of FIFO is full
            7 => 4, // Interrupt when 7/8 of FIFO is full
            _ => panic!(),
        }
    }
}
