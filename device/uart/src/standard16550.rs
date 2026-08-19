mod baud;
mod buffer;
mod fifo;
mod interrupt;
mod line;
mod modem;

use super::Parity;

/// # References
/// * [Table of Registers](https://www.lookrs232.com/rs232/registers.htm)
#[io::registers]
struct Registers {
    buffer_or_baud_low: BufferOrBaudLow,
    interrupt_enable_or_baud_high: InterruptEnableOrBaudHigh,
    fifo_control_or_interrupt_identification: FifoControlOrInterruptIdentification,
    line_control: line::Control,
    modem_control: modem::Control,
    line_status: line::Status,
    modem_status: modem::Status,
}

impl RegistersAccessor {
    fn can_send_byte(&self) -> bool {
        unsafe { self.read_line_status() }.read_empty_transmitter_bit()
    }

    fn disable_all_interrupts(&mut self) {
        if self.is_baud_rate_setting_mode() {
            self.set_baud_rate_setting_mode(false);
        }
        unsafe {
            self.write_interrupt_enable_or_baud_high(
                InterruptEnableOrBaudHigh::write_interrupt_enable(interrupt::Enable::default()),
            );
        }
    }

    fn is_baud_rate_setting_mode(&self) -> bool {
        unsafe { self.read_line_control() }.read_divisor_latch_access_bit()
    }

    fn set_fifo(
        &mut self,
        enable: bool,
        clear_receive: bool,
        clear_transmit: bool,
        dma: bool,
        enable_64byte: bool,
        interrupt_trigger_bytes: u8,
    ) {
        unsafe {
            self.write_fifo_control_or_interrupt_identification(
                FifoControlOrInterruptIdentification::write_fifo_control(fifo::Control::set(
                    enable,
                    clear_receive,
                    clear_transmit,
                    dma,
                    enable_64byte,
                    interrupt_trigger_bytes,
                )),
            );
        }
    }

    fn set_line_control(
        &mut self,
        parity: Option<Parity>,
        send_break: bool,
        stop_bit: u8,
        word_length: u8,
    ) {
        unsafe {
            self.write_line_control(line::Control::set(
                parity,
                send_break,
                stop_bit,
                word_length,
            ));
        }
    }

    fn set_modem(
        &mut self,
        force_data_terminal_ready: bool,
        force_request_to_send: bool,
        out1: bool,
        out2: bool,
        loopback_mode: bool,
        autoflow_control: bool,
    ) {
        unsafe {
            self.write_modem_control(modem::Control::set(
                force_data_terminal_ready,
                force_request_to_send,
                out1,
                out2,
                loopback_mode,
                autoflow_control,
            ));
        }
    }

    fn send_byte(&mut self, data: u8) {
        if self.is_baud_rate_setting_mode() {
            self.set_baud_rate_setting_mode(false);
        }
        unsafe {
            self.write_buffer_or_baud_low(BufferOrBaudLow::write_buffer(buffer::Register::send(
                data,
            )));
        }
    }

    fn set_baud_rate_divisor(&mut self, baud_rate_divisor: u16) {
        if !self.is_baud_rate_setting_mode() {
            self.set_baud_rate_setting_mode(true);
        }
        let baud_rate_divisor_low: u8 = (baud_rate_divisor & 0x00ff) as u8;
        let baud_rate_divisor_high: u8 = (baud_rate_divisor >> u8::BITS) as u8;
        unsafe {
            self.write_buffer_or_baud_low(BufferOrBaudLow::write_baud_low(baud::Low::set(
                baud_rate_divisor_low,
            )));
            self.write_interrupt_enable_or_baud_high(InterruptEnableOrBaudHigh::write_baud_high(
                baud::High::set(baud_rate_divisor_high),
            ));
        }
    }

    fn set_baud_rate_setting_mode(&mut self, value: bool) {
        unsafe {
            self.write_line_control(
                self.read_line_control()
                    .update_divisor_latch_access_bit(value),
            );
        }
    }
}

#[io::overlap]
union BufferOrBaudLow {
    buffer: buffer::Register,
    baud_low: baud::Low,
}

#[io::overlap]
union InterruptEnableOrBaudHigh {
    interrupt_enable: interrupt::Enable,
    baud_high: baud::High,
}

#[io::overlap]
union FifoControlOrInterruptIdentification {
    fifo_control: fifo::Control,
    interrupt_identification: interrupt::Identification,
}
