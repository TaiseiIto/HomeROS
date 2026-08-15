/// # References
/// * [FIFO Control Register](https://www.lookrs232.com/rs232/fcr.htm)
#[io::register]
struct Control {
    enable: bool,
    clear_receive: bool,
    clear_transmit: bool,
    dma: bool,
    __: bool,
    enable_64byte: bool,
    interrupt_trigger_level: [bool; 2],
}
