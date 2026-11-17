/// # References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual](https://cdrdv2.intel.com/v1/dl/getContent/671200) 3.13.5.1 Local Vector Table, Figure 13-8. Local Vector Table (LVT)
#[io::register]
pub struct Register {
    vector: u8,
    delivery_mode: [bool; 3],
    __: bool,
    delivery_status: bool,
    interrupt_input_pin_polarity: bool,
    remote_irr: bool,
    trigger_mode: bool,
    mask: bool,
    timer_mode: [bool; 2],
    __: [bool; 13],
}
