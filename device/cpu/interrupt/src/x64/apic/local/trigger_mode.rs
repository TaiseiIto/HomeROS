/// # References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual](https://cdrdv2.intel.com/v1/dl/getContent/671200) 13.8.4 Interrupt Acceptance for Fixed Interrupts
#[io::register]
pub struct Register {
    trigger_mode: u32,
}
