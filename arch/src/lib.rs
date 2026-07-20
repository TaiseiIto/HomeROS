#![no_std]

use core::arch::asm;

#[inline(always)]
pub unsafe fn wait_for_interrupt() {
    unsafe {
        #[cfg(target_arch = "riscv64")]
        asm!("wfi");
        #[cfg(target_arch = "aarch64")]
        asm!("wfi");
        #[cfg(target_arch = "x86_64")]
        asm!("hlt");
    }
}
