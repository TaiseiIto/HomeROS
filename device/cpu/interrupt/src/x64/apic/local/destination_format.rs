/// # References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual](https://cdrdv2.intel.com/v1/dl/getContent/671200) 13.6.2.2 Logical Destination Mode
#[io::register]
pub struct Register {
    __: [bool; 28],
    model: [bool; 4],
    __: u32,
}
