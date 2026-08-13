#![no_std]

pub mod fifo;
pub mod interrupt;
pub mod line;

use core::arch::asm;

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
