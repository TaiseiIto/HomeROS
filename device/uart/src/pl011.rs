mod baud_rate;
mod control;
mod data;
mod dma_control;
mod flag;
mod interrupt;
mod irda_low_power_counter;
mod line_control;
mod peripheral;
mod prime_cell;
mod receive_status;

/// # References
/// * [ARM PrimeCell UART (PL011) Technical Reference Manual](https://support.arm.com/documentation/ddi0183/g/programmers-model/summary-of-registers?lang=en)
#[io::registers]
pub struct Registers {
    data: data::Register,
    receive_status: receive_status::Register,
    __: [u8; 0x18 - 0x08],
    flag: flag::Register,
    __: [u8; 0x20 - 0x1c],
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
    __: [u8; 0xfe0 - 0x4c],
    peripheral_id0: peripheral::Id0,
    peripheral_id1: peripheral::Id1,
    peripheral_id2: peripheral::Id2,
    peripheral_id3: peripheral::Id3,
    prime_cell_id0: prime_cell::Id0,
    prime_cell_id1: prime_cell::Id1,
    prime_cell_id2: prime_cell::Id2,
    prime_cell_id3: prime_cell::Id3,
}

impl RegistersAccessor {
    fn can_send_character(&self) -> bool {
        !unsafe { self.read_flag() }.read_busy_bit()
    }

    fn send_byte(&mut self, byte: u8) {
        unsafe {
            self.write_data(data::RegisterPretty::default().update_data_u8(byte));
        }
    }

    fn set_baud_rate_divisor(&mut self, baud_rate_divisor: u16) {
        unsafe {
            self.write_integer_baud_rate(
                baud_rate::integer::RegisterPretty::default().update_divisor_u16(baud_rate_divisor),
            );
        }
    }
}
