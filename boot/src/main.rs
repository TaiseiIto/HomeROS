#![no_main]
#![no_std]

use {arch::wait_for_interrupt, core::panic::PanicInfo};

#[cfg(firmware = "uefi")]
use firmware::uefi;

#[cfg(target_arch = "riscv64")]
use core::arch::naked_asm;

#[cfg(target_arch = "riscv64")]
#[unsafe(link_section = ".text._start")]
#[unsafe(naked)]
#[unsafe(no_mangle)]
unsafe extern "C" fn _start() -> ! {
    naked_asm!("la sp, _stack_bottom", "j initialize_global");
}

#[cfg(target_arch = "riscv64")]
#[unsafe(no_mangle)]
fn initialize_global() {
    main(unsafe { firmware::Global::new() });
    unreachable!();
}

/// # References
/// * [EFI_IMAGE_ENTRY_POINT](https://uefi.org/specs/UEFI/2.11/04_EFI_System_Table.html#efi-image-entry-point)
#[cfg(firmware = "uefi")]
#[must_use]
#[unsafe(no_mangle)]
extern "efiapi" fn efi_main(
    image_handle: uefi::HandleMut,
    system_table: *mut uefi::system::Table,
) -> uefi::Status {
    main(unsafe { firmware::Global::new(image_handle, system_table) });
    unreachable!();
}

fn main(global: firmware::Global) {
    unsafe {
        global.set();
    }
    unimplemented!();
}

#[panic_handler]
fn panic(panic: &PanicInfo) -> ! {
    firmware::println!("{}", panic);
    loop {
        unsafe {
            wait_for_interrupt();
        }
    }
    unreachable!();
}
