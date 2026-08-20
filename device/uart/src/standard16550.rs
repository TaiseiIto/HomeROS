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

    fn enable_interrupts(
        &mut self,
        received_data_available: bool,
        transmitter_holding_register_empty: bool,
        receiver_line_status: bool,
        modem_status: bool,
        sleep_mode: bool,
        low_power_mode: bool,
    ) {
        if self.is_baud_rate_setting_mode() {
            self.set_baud_rate_setting_mode(false);
        }
        unsafe {
            self.write_interrupt_enable_or_baud_high(
                InterruptEnableOrBaudHigh::write_interrupt_enable(interrupt::Enable::new(
                    received_data_available,
                    transmitter_holding_register_empty,
                    receiver_line_status,
                    modem_status,
                    sleep_mode,
                    low_power_mode,
                )),
            );
        }
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
        let enable_received_data_available_interrupt: bool = false;
        let enable_transmitter_holding_register_empty_interrupt: bool = false;
        let enable_receiver_line_status_interrupt: bool = false;
        let enable_modem_status_interrupt: bool = false;
        let enable_sleep_mode_interrupt: bool = false;
        let enable_low_power_mode_interrupt: bool = false;
        let clear_receive: bool = true;
        let clear_transmit: bool = true;
        let dma: bool = false;
        let enable_64byte: bool = false;
        let interrupt_trigger_bytes: u8 = 14;
        let force_data_terminal_ready: bool = true;
        let force_request_to_send: bool = true;
        let out1: bool = true;
        let out2: bool = true;
        let loopback_mode: bool = false;
        let autoflow_control: bool = false;
        self.enable_interrupts(
            enable_received_data_available_interrupt,
            enable_transmitter_holding_register_empty_interrupt,
            enable_receiver_line_status_interrupt,
            enable_modem_status_interrupt,
            enable_sleep_mode_interrupt,
            enable_low_power_mode_interrupt,
        );
        self.set_baud_rate(baud_rate);
        self.set_line(parity, send_break, stop_bits, word_bits);
        self.set_fifo(
            enable_fifo,
            clear_receive,
            clear_transmit,
            dma,
            enable_64byte,
            interrupt_trigger_bytes,
        );
        self.set_modem(
            force_data_terminal_ready,
            force_request_to_send,
            out1,
            out2,
            loopback_mode,
            autoflow_control,
        );
    }

    fn is_baud_rate_setting_mode(&self) -> bool {
        unsafe { self.read_line_control() }.read_divisor_latch_access_bit()
    }

    fn set_baud_rate(&mut self, baud_rate: usize) {
        let frequency: usize = 115200;
        let divisor: u16 = (frequency / baud_rate) as u16;
        if !self.is_baud_rate_setting_mode() {
            self.set_baud_rate_setting_mode(true);
        }
        let divisor_low: u8 = (divisor & 0x00ff) as u8;
        let divisor_high: u8 = (divisor >> u8::BITS) as u8;
        unsafe {
            self.write_buffer_or_baud_low(BufferOrBaudLow::write_baud_low(baud::Low::new(
                divisor_low,
            )));
            self.write_interrupt_enable_or_baud_high(InterruptEnableOrBaudHigh::write_baud_high(
                baud::High::new(divisor_high),
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
                FifoControlOrInterruptIdentification::write_fifo_control(fifo::Control::new(
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

    fn set_line(&mut self, parity: Option<Parity>, send_break: bool, stop_bits: u8, word_bits: u8) {
        unsafe {
            self.write_line_control(line::Control::new(parity, send_break, stop_bits, word_bits));
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
            self.write_modem_control(modem::Control::new(
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
            self.write_buffer_or_baud_low(BufferOrBaudLow::write_buffer(buffer::Register::new(
                data,
            )));
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
