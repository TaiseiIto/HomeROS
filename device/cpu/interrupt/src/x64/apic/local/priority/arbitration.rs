/// # References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual](https://cdrdv2.intel.com/v1/dl/getContent/671200) 13.6.2.4 Lowest Priority Delivery Mode
#[io::register]
pub struct Register {
    sub_class: [bool; 4],
    class: [bool; 4],
    __: [u8; 3],
}
