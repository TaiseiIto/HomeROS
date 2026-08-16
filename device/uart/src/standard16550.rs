mod baud;
mod buffer;
mod fifo;
mod interrupt;
mod line;
mod modem;

use core::mem::ManuallyDrop;

#[repr(C)]
union BufferOrBaudLow {
    buffer: ManuallyDrop<buffer::Register>,
    baud_low: ManuallyDrop<baud::Low>,
}

#[repr(C)]
union InterruptEnableOrBaudHigh {
    interrupt_enable: ManuallyDrop<interrupt::Enable>,
    baud_high: ManuallyDrop<baud::High>,
}
