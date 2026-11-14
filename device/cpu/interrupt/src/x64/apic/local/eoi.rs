/// # References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual](https://cdrdv2.intel.com/v1/dl/getContent/671200) 13.8.5 Signaling Interrupt Servicing Completion, Figure 13-21. EOI Register
#[io::register]
pub struct Register {
    eoi: u32,
}
