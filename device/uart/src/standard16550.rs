mod baud;
mod buffer;
mod fifo;
mod interrupt;
mod line;
mod modem;

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
    fn divisor_latch_access(&self) -> bool {
        unsafe { self.read_line_control() }.read_divisor_latch_access_bit()
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
