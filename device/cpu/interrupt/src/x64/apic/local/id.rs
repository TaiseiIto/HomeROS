/// # References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual](https://cdrdv2.intel.com/v1/dl/getContent/671200) 13.4.6 Local APIC ID
#[io::register]
pub struct Register {
    __: [bool; 24],
    id: u8,
}
