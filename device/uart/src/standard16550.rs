mod baud;
mod buffer;
mod fifo;
mod interrupt;
mod line;
mod modem;

#[io::overlap]
#[repr(C)]
union BufferOrBaudLow {
    buffer: core::mem::ManuallyDrop<buffer::Register>,
    baud_low: core::mem::ManuallyDrop<baud::Low>,
}

#[io::overlap]
#[repr(C)]
union InterruptEnableOrBaudHigh {
    interrupt_enable: core::mem::ManuallyDrop<interrupt::Enable>,
    baud_high: core::mem::ManuallyDrop<baud::High>,
}
