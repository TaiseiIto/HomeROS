/// # References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual](https://cdrdv2.intel.com/v1/dl/getContent/671200) 13.4.8 Local APIC Version Register
#[io::register]
pub struct Register {
    version: u8,
    __: u8,
    max_lvt_entry: u8,
    eoi_broadcast_suppression: bool,
    __: [bool; 7],
}
