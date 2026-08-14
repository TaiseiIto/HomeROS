/// # References
/// * [FIFO Control Register](https://www.lookrs232.com/rs232/fcr.htm)
#[bit::field]
struct Control {
    enable: bool,
    clear_receive: bool,
    clear_transmit: bool,
    dma: bool,
    __: bool,
    enable_64byte: bool,
    interrupt_trigger_level: [bool; 2],
}

impl Control {
    fn interrupt_trigger_bytes(&self) -> u8 {
        match self.interrupt_trigger_level_bits_read() {
            [false, false] => 1,
            [true, false] => 4,
            [false, true] => 8,
            [true, true] => 14,
        }
    }
}
