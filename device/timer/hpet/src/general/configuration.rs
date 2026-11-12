/// # References
/// * [IA-PC HPET (High Precision Event Timer) Specification](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/software-developers-hpet-spec-1-0a.pdf) 2.3.5 General Configuration Register
#[io::register]
pub struct Register {
    enable_cnf: bool,
    leb_rt_chf: bool,
    __: [bool; 6],
    __: u8,
    __: u16,
    __: u32,
}
