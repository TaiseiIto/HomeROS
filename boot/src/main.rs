#![no_main]
#![no_std]

use {arch::wait_for_interrupt, core::panic::PanicInfo};

#[cfg(firmware = "uefi")]
use firmware::uefi::{Handle, system::Table};

#[cfg(target_arch = "riscv64")]
use core::arch::naked_asm;

#[cfg(target_arch = "riscv64")]
#[unsafe(link_section = ".text._start")]
#[unsafe(naked)]
#[unsafe(no_mangle)]
unsafe extern "C" fn _start() -> ! {
    naked_asm!("la sp, _stack_bottom", "j main");
}

/// # References
/// * [EFI_IMAGE_ENTRY_POINT](https://uefi.org/specs/UEFI/2.11/04_EFI_System_Table.html#efi-image-entry-point)
#[cfg(firmware = "uefi")]
#[unsafe(no_mangle)]
fn efi_main(_image_handle: Handle, _system_table: *const Table) {
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
