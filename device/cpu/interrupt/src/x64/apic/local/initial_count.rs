/// # References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual](https://cdrdv2.intel.com/v1/dl/getContent/671200) 3.13.5.4 APIC Timer, Figure 13-11. Initial Count and Current Count Register
#[io::register]
pub struct Register {
    value: u32,
}
