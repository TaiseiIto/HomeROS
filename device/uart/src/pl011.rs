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

use super::Parity;

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
    fn can_send_byte(&self) -> bool {
        !unsafe { self.read_flag() }.read_busy_bit()
    }

    fn enable_interrupt(
        &mut self,
        ri_modem: bool,
        cts_modem: bool,
        dcd_modem: bool,
        dsr_modem: bool,
        receive: bool,
        transmit: bool,
        receive_timeout: bool,
        framing_error: bool,
        parity_error: bool,
        break_erro: bool,
        overrun_error: bool,
    ) {
        unsafe {
            self.write_interrupt_mask(interrupt::Register::new(
                !ri_modem,
                !cts_modem,
                !dcd_modem,
                !dsr_modem,
                !receive,
                !transmit,
                !receive_timeout,
                !framing_error,
                !parity_error,
                !break_erro,
                !overrun_error,
            ));
        }
    }

    fn send_byte(&mut self, byte: u8) {
        unsafe {
            self.write_data(data::Register::default().update_data_u8(byte));
        }
    }

    fn set_baud_rate_divisor(&mut self, baud_rate_divisor: u16) {
        unsafe {
            self.write_integer_baud_rate(
                baud_rate::integer::Register::default().update_divisor_u16(baud_rate_divisor),
            );
        }
    }

    fn set_control(
        &mut self,
        uart_enable: bool,
        sir_enable: bool,
        sir_low_power_irda_mode: bool,
        loopback_enable: bool,
        transmit_enable: bool,
        receive_enable: bool,
        data_transmit_ready: bool,
        request_to_send: bool,
        out1: bool,
        out2: bool,
        rts_enable: bool,
        cts_enable: bool,
    ) {
        unsafe {
            self.write_control(control::Register::new(
                uart_enable,
                sir_enable,
                sir_low_power_irda_mode,
                loopback_enable,
                transmit_enable,
                receive_enable,
                data_transmit_ready,
                request_to_send,
                out1,
                out2,
                rts_enable,
                cts_enable,
            ));
        }
    }

    fn set_fifo(&mut self, transmit_ratio_8times: u8, receive_ratio_8times: u8) {
        unsafe {
            self.write_interrupt_fifo_level(interrupt::fifo_level::Register::new(
                transmit_ratio_8times,
                receive_ratio_8times,
            ));
        }
    }

    fn set_line_control(
        &mut self,
        enable_fifo: bool,
        parity: Option<Parity>,
        send_break: bool,
        stop_bit: u8,
        word_length: u8,
    ) {
        unsafe {
            self.write_line_control(line_control::Register::new(
                enable_fifo,
                parity,
                send_break,
                stop_bit,
                word_length,
            ));
        }
    }
}
