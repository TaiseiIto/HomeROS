/// # References
/// * [IA-PC HPET (High Precision Event Timer) Specification](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/software-developers-hpet-spec-1-0a.pdf) 2.3.4 General Capabilities and ID Register
#[io::register]
pub struct Register {
    rev_id: u8,
    num_tim_cap: [bool; 5],
    count_size_cap: bool,
    __: bool,
    leg_rt_cap: bool,
    vendor_id: u16,
    counter_clk_reriod: u32,
}
