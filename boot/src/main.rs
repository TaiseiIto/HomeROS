#![no_main]
#![no_std]

use {arch::wait_for_interrupt, core::panic::PanicInfo};

#[cfg(firmware = "uefi")]
use firmware::uefi;

#[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
use core::arch::naked_asm;

#[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
#[unsafe(link_section = ".text._start")]
#[unsafe(naked)]
#[unsafe(no_mangle)]
unsafe extern "C" fn _start() -> ! {
    #[cfg(target_arch = "aarch64")]
    naked_asm!(
        "ldr x9, =_stack_bottom",
        "mov sp, x9",
        "b initialize_global"
    );
    #[cfg(target_arch = "riscv64")]
    naked_asm!("la sp, _stack_bottom", "j initialize_global");
}

#[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
#[unsafe(no_mangle)]
fn initialize_global() {
    main(unsafe { firmware::Global::new() });
    unreachable!();
}

/// # References
/// * [EFI_IMAGE_ENTRY_POINT](https://uefi.org/specs/UEFI/2.11/04_EFI_System_Table.html#efi-image-entry-point)
#[cfg(firmware = "uefi")]
#[unsafe(no_mangle)]
extern "efiapi" fn efi_main(
    image_handle: uefi::HandleMut,
    system_table: *mut uefi::system::Table,
) -> uefi::Status {
    main(unsafe { firmware::Global::new(image_handle, system_table) });
    unreachable!();
}

fn main(global: firmware::Global) {
    global.set();
    uart::initialize();
    firmware::println!("Hello, firmware!");
    uart::println!("Hello, UART!");
    uart::println!(
        "firmware::GLOBAL = {:#x?}",
        firmware::GLOBAL.lock().get_mut().unwrap()
    );
    unimplemented!();
}

#[panic_handler]
fn panic(panic: &PanicInfo) -> ! {
    uart::dbg!(panic);
    loop {
        unsafe {
            wait_for_interrupt();
        }
    }
}
