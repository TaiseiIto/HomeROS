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

use super::{Driver, Parity};

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
    interrupt_clear: interrupt::Register,
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
    fn clear_all_interrupts(&mut self) {
        let ri_modem: bool = true;
        let cts_modem: bool = true;
        let dcd_modem: bool = true;
        let dsr_modem: bool = true;
        let receive: bool = true;
        let transmit: bool = true;
        let receive_timeout: bool = true;
        let framing_error: bool = true;
        let parity_error: bool = true;
        let break_erro: bool = true;
        let overrun_error: bool = true;
        self.clear_interrupts(
            ri_modem,
            cts_modem,
            dcd_modem,
            dsr_modem,
            receive,
            transmit,
            receive_timeout,
            framing_error,
            parity_error,
            break_erro,
            overrun_error,
        );
    }

    fn clear_interrupts(
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
            self.write_interrupt_clear(interrupt::Register::new(
                ri_modem,
                cts_modem,
                dcd_modem,
                dsr_modem,
                receive,
                transmit,
                receive_timeout,
                framing_error,
                parity_error,
                break_erro,
                overrun_error,
            ));
        }
    }

    fn disable(&mut self) {
        let uart_enable: bool = false;
        let sir_enable: bool = false;
        let sir_low_power_irda_mode: bool = false;
        let loopback_enable: bool = false;
        let transmit_enable: bool = false;
        let receive_enable: bool = false;
        let data_transmit_ready: bool = false;
        let request_to_send: bool = false;
        let out1: bool = false;
        let out2: bool = false;
        let rts_enable: bool = false;
        let cts_enable: bool = false;
        self.set_control(
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
        );
    }

    fn disable_all_interrupts(&mut self) {
        let ri_modem: bool = false;
        let cts_modem: bool = false;
        let dcd_modem: bool = false;
        let dsr_modem: bool = false;
        let receive: bool = false;
        let transmit: bool = false;
        let receive_timeout: bool = false;
        let framing_error: bool = false;
        let parity_error: bool = false;
        let break_erro: bool = false;
        let overrun_error: bool = false;
        self.enable_interrupts(
            ri_modem,
            cts_modem,
            dcd_modem,
            dsr_modem,
            receive,
            transmit,
            receive_timeout,
            framing_error,
            parity_error,
            break_erro,
            overrun_error,
        );
    }

    fn enable_interrupts(
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
                ri_modem,
                cts_modem,
                dcd_modem,
                dsr_modem,
                receive,
                transmit,
                receive_timeout,
                framing_error,
                parity_error,
                break_erro,
                overrun_error,
            ));
        }
    }

    fn set_baud_rate(&mut self, baud_rate: usize) {
        let frequency: usize = 24000000;
        let integer_baud_rate: usize = frequency / (16 * baud_rate);
        let fractional_baud_rate: usize = 4 * frequency / baud_rate - 64 * integer_baud_rate;
        unsafe {
            self.write_integer_baud_rate(baud_rate::integer::Register::new(
                integer_baud_rate as u16,
            ));
            self.write_fractional_baud_rate(baud_rate::fractional::Register::new(
                fractional_baud_rate as u32,
            ));
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
        stop_bits: u8,
        word_bits: u8,
    ) {
        unsafe {
            self.write_line_control(line_control::Register::new(
                enable_fifo,
                parity,
                send_break,
                stop_bits,
                word_bits,
            ));
        }
    }
}

impl Driver for RegistersAccessor {
    fn can_send_byte(&self) -> bool {
        !unsafe { self.read_flag() }.read_busy_bit()
    }

    fn initialize(
        &mut self,
        baud_rate: usize,
        enable_fifo: bool,
        parity: Option<Parity>,
        send_break: bool,
        stop_bits: u8,
        word_bits: u8,
    ) {
        let uart_enable: bool = true;
        let sir_enable: bool = false;
        let sir_low_power_irda_mode: bool = false;
        let loopback_enable: bool = false;
        let transmit_enable: bool = true;
        let receive_enable: bool = true;
        let data_transmit_ready: bool = false;
        let request_to_send: bool = false;
        let out1: bool = false;
        let out2: bool = false;
        let rts_enable: bool = false;
        let cts_enable: bool = false;
        self.disable();
        self.disable_all_interrupts();
        self.clear_all_interrupts();
        self.set_baud_rate(baud_rate);
        self.set_line_control(enable_fifo, parity, send_break, stop_bits, word_bits);
        self.set_control(
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
        );
    }

    unsafe fn send_byte_unchecked(&mut self, byte: u8) {
        unsafe {
            self.write_data(data::Register::default().update_data_u8(byte));
        }
    }
}
