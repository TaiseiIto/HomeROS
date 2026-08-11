#![no_std]

use core::arch::asm;

pub fn write_string(string: &str) {
    for byte in string.bytes() {
        write_byte(byte);
    }
}

fn write_byte(byte: u8) {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        asm!("out dx, al", in("dx") 0x02f8, in("al") byte);
    }
}
