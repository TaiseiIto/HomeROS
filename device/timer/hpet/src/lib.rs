#![no_std]

mod general;
mod timer;

use core::slice::from_raw_parts;

/// # References
/// * [IA-PC HPET (High Precision Event Timer) Specification](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/software-developers-hpet-spec-1-0a.pdf) 2.3.1 Register Overview
/// # TODO
/// * Print all registers.
#[io::registers]
#[debug_function(timers)]
pub struct Registers {
    general_capabilities_and_id: general::capabilities_and_id::Register,
    __: u64,
    general_configuration: general::configuration::Register,
    __: u64,
    general_interrupt_status: general::interrupt_status::Register,
    __: [u8; 0xef - 0x28 + 1],
    #[unregister]
    main_counter_register: u64,
    __: u64,
}

impl Registers {
    fn number_of_timers(&self) -> usize {
        unsafe { self.read_general_capabilities_and_id_memory() }.read_num_tim_cap_shift() as usize
    }

    fn timers(&self) -> &[timer::Registers] {
        let number_of_timers: usize = self.number_of_timers();
        let timers: *const timer::Registers =
            unsafe { (self as *const Registers).add(1) } as *const timer::Registers;
        unsafe { from_raw_parts(timers, number_of_timers) }
    }
}
