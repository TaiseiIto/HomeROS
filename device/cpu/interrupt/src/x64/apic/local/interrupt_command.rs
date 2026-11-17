/// # References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual](https://cdrdv2.intel.com/v1/dl/getContent/671200) 3.13.6.1 Interrupt Command Register (ICR), Figure 13-12. Interrupt Command Register (ICR)
#[io::register]
pub struct Low {
    vector: u8,
    delivery_mode: [bool; 3],
    destination_mode: bool,
    delivery_status: bool,
    __: bool,
    level: bool,
    trigger_mode: bool,
    __: [bool; 2],
    destination_shorthand: [bool; 2],
    __: [bool; 12],
}

/// # References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual](https://cdrdv2.intel.com/v1/dl/getContent/671200) 3.13.6.1 Interrupt Command Register (ICR), Figure 13-12. Interrupt Command Register (ICR)
#[io::register]
pub struct High {
    __: [u8; 3],
    destination_field: u8,
}
