#![no_main]
#![no_std]

use {arch::wait_for_interrupt, core::panic::PanicInfo};

#[cfg(target_arch = "riscv64")]
use core::arch::naked_asm;

#[cfg(target_arch = "riscv64")]
#[unsafe(link_section = ".text._start")]
#[unsafe(naked)]
#[unsafe(no_mangle)]
unsafe extern "C" fn _start() -> ! {
    naked_asm!("la sp, _stack_bottom", "j main");
}

#[cfg(firmware = "uefi")]
#[unsafe(no_mangle)]
fn efi_main() {
    main();
}

#[unsafe(no_mangle)]
fn main() {
    unimplemented!();
}

#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {
        unsafe {
            wait_for_interrupt();
        }
    }
}
