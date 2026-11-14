/// # References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual](https://cdrdv2.intel.com/v1/dl/getContent/671200) 13.9 Spurious Interrupt, Figure 13-23. Spurious-Interrupt Vector Register (SVR)
#[io::register]
pub struct Register {
    spurious_vector: u8,
    apic_software: bool,
    focus_processor_checking: bool,
    __: [bool; 2],
    eoi_broadcast_suppression: bool,
    __: [bool; 51],
}
