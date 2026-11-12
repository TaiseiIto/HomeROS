/// # References
/// * [IA-PC HPET (High Precision Event Timer) Specification](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/software-developers-hpet-spec-1-0a.pdf) 2.3.6 General Interrupt Status Register
#[io::register]
pub struct Register {
    timer_n_interrupt_active: u32,
    __: u32,
}
