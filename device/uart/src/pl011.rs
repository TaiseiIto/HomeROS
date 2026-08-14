mod baud_rate;
mod control;
mod data;
mod dma_control;
mod flag;
mod interrupt;
mod irda_low_power_counter;
mod line_control;
mod peripheral;
mod receive_status;

/// # References
/// * [ARM PrimeCell UART (PL011) Technical Reference Manual](https://support.arm.com/documentation/ddi0183/g/programmers-model/summary-of-registers?lang=en)
#[derive(Debug)]
#[repr(packed)]
pub struct Registers {
    data: data::Register,
    receive_status: receive_status::Register,
    reserved0: [u8; 0x18 - 0x08],
    flag: flag::Register,
    reserved1: [u8; 0x20 - 0x1c],
    irda_low_power_counter: irda_low_power_counter::Register,
    integer_baud_rate: baud_rate::integer::Register,
    fractional_baud_rate: baud_rate::fractional::Register,
    line_control: line_control::Register,
    control: control::Register,
    interrupt_fifo_level: interrupt::fifo_level::Register,
    interrupt_mask: interrupt::Register,
    raw_interrupt_status: interrupt::Register,
    masked_interrupt_status: interrupt::Register,
    interupt_clear: interrupt::Register,
    dma_control: dma_control::Register,
    reserved2: [u8; 0xfe0 - 0x4c],
    peripheral_id0: peripheral::Id0,
    peripheral_id1: peripheral::Id1,
    peripheral_id2: peripheral::Id2,
    peripheral_id3: peripheral::Id3,
}
