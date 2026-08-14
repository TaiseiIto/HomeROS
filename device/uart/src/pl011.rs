mod baud_rate;
mod control;
mod data;
mod flag;
mod interrupt;
mod irda_low_power_counter;
mod line_control;
mod receive_status;

/// # References
/// * [ARM PrimeCell UART (PL011) Technical Reference Manual](https://support.arm.com/documentation/ddi0183/g/programmers-model/summary-of-registers?lang=en)
#[derive(Debug)]
#[repr(packed)]
pub struct Registers {
    data: data::Register,
    receive_status: receive_status::Register,
    reserved0: [u32; 4],
    flag: flag::Register,
    reserved1: u32,
    irda_low_power_counter: irda_low_power_counter::Register,
    integer_baud_rate: baud_rate::integer::Register,
    fractional_baud_rate: baud_rate::fractional::Register,
    line_control: line_control::Register,
    control: control::Register,
    interrupt_fifo_level: interrupt::fifo_level::Register,
    interrupt_mask: interrupt::mask::Register,
}
