mod comparator;
mod configuration_and_capability;
mod fsb_interrupt_route;

/// # References
/// * [IA-PC HPET (High Precision Event Timer) Specification](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/software-developers-hpet-spec-1-0a.pdf) 2.3.1 Register Overview
#[io::registers]
pub struct Registers {
    configuration_and_capability: configuration_and_capability::Register,
    comparator_value: comparator::Register,
    fsb_interrupt_route: fsb_interrupt_route::Register,
    __: u64,
}
