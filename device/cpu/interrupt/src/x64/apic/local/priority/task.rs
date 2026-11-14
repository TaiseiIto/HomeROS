/// # References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual](https://cdrdv2.intel.com/v1/dl/getContent/671200) 13.8.6 Task Priority in IA-32e Mode
/// # TODO
/// * Merge reserved bits.
#[io::register]
pub struct Register {
    zero: [bool; 4],
    cr8: [bool; 4],
    __: u32,
    __: u16,
    __: u8,
}
