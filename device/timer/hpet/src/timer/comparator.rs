/// # References
/// * [IA-PC HPET (High Precision Event Timer) Specification](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/software-developers-hpet-spec-1-0a.pdf) 2.3.8 Timer N Configuration and Capabilities Register
#[io::register]
pub struct Register {
    value: u32,
    __: u32,
}
