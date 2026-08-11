#![no_std]

use core::arch::asm;

pub fn write_string(string: &str) {
    for byte in string.bytes() {
        write_byte(byte);
    }
}

/// # TODO
/// This is an ad hoc implementation working on only QEMU.
/// Implement RS232C completely.
fn write_byte(byte: u8) {
    unsafe {
        #[cfg(target_arch = "aarch64")]
        asm!("strb w9, x10", in("w9") byte, in("x10") 0x09000000);
        #[cfg(target_arch = "riscv64")]
        asm!("sb t0, t1", in("t0") byte, in("t2") 0x10000000);
        #[cfg(target_arch = "x86_64")]
        asm!("out dx, al", in("dx") 0x03f8, in("al") byte);
    }
}
