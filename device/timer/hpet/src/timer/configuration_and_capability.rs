/// # References
/// * [IA-PC HPET (High Precision Event Timer) Specification](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/software-developers-hpet-spec-1-0a.pdf) 2.3.8 Timer N Configuration and Capabilities Register
#[io::register]
pub struct Register {
    __: bool,
    int_type_cnf: bool,
    int_enb_cnf: bool,
    type_cnf: bool,
    per_int_cap: bool,
    size_cap: bool,
    val_set_cnf: bool,
    __: bool,
    mode32_cnf: bool,
    int_route_cnf: [bool; 5],
    fsb_en_cnf: bool,
    fsb_int_del_cap: bool,
    __: u16,
    int_route_cap: u32,
}
