/// # References
/// * [IA-PC HPET (High Precision Event Timer) Specification](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/software-developers-hpet-spec-1-0a.pdf) 2.3.10 Timer N FSB Interrupt Route Register
#[io::register]
pub struct Register {
    int_val: u32,
    int_addr: u32,
}
