/// # References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual](https://cdrdv2.intel.com/v1/dl/getContent/671200) 3.13.5.4 APIC Timer, Figure 13-10. Divide Configuration Register
#[io::register]
pub struct Register {
    low: [bool; 2],
    __: bool,
    high: bool,
    __: [bool; 28],
}
