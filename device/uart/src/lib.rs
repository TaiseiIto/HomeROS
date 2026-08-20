#![no_std]

#[cfg(uart = "pl011")]
mod pl011;
#[cfg(uart = "16550")]
mod standard16550;

use core::arch::asm;

pub enum Parity {
    Even,
    High,
    Low,
    Odd,
}

/// # TODO
/// This is an ad hoc implementation working on only QEMU.
/// Implement RS232C completely.
pub fn write_byte(byte: u8) {
    unsafe {
        #[cfg(target_arch = "aarch64")]
        asm!("strb {byte:w}, [{address:x}]", byte = in(reg) byte, address = in(reg) 0x09000000);
        #[cfg(target_arch = "riscv64")]
        asm!("sb {byte}, ({address})", byte = in(reg) byte, address = in(reg) 0x10000000);
        #[cfg(target_arch = "x86_64")]
        asm!("out dx, al", in("dx") 0x03f8, in("al") byte);
    }
}

pub trait Driver {
    fn can_send_byte(&self) -> bool;
    fn initialize(
        &mut self,
        baud_rate: usize,
        enable_fifo: bool,
        parity: Option<Parity>,
        send_break: bool,
        stop_bits: u8,
        word_bits: u8,
    );
    unsafe fn send_byte_unchecked(&mut self, data: u8);

    fn send_byte(&mut self, data: u8) {
        while !self.can_send_byte() {
            arch::pause();
        }
        unsafe {
            self.send_byte_unchecked(data);
        }
    }
}
