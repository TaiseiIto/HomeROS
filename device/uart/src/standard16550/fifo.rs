/// # References
/// * [FIFO Control Register](https://www.lookrs232.com/rs232/fcr.htm)
#[io::register]
pub struct Control {
    enable: bool,
    clear_receive: bool,
    clear_transmit: bool,
    dma: bool,
    __: bool,
    enable_64byte: bool,
    interrupt_trigger_level: [bool; 2],
}

impl Control {
    pub fn new(
        enable: bool,
        clear_receive: bool,
        clear_transmit: bool,
        dma: bool,
        enable_64byte: bool,
        interrupt_trigger_bytes: u8,
    ) -> Self {
        Self::default()
            .update_enable_bit(enable)
            .update_clear_receive_bit(clear_receive)
            .update_clear_transmit_bit(clear_transmit)
            .update_dma_bit(dma)
            .update_enable_64byte_bit(enable_64byte)
            .update_interrupt_trigger_level_shift(match interrupt_trigger_bytes {
                1 => 0,  // Interrupt when FIFO has 1 bytes
                4 => 1,  // Interrupt when FIFO has 4 bytes
                8 => 2,  // Interrupt when FIFO has 8 bytes
                14 => 3, // Interrupt when FIFO has 14 bytes
                _ => panic!(),
            })
    }

    pub fn update_interrupt_trigger_bytes(self, bytes: u8) -> Self {
        self.update_interrupt_trigger_level_shift(match bytes {
            1 => 0,  // Interrupt when FIFO has 1 bytes
            4 => 1,  // Interrupt when FIFO has 4 bytes
            8 => 2,  // Interrupt when FIFO has 8 bytes
            14 => 3, // Interrupt when FIFO has 14 bytes
            _ => panic!(),
        })
    }
}
