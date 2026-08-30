#![no_main]
#![no_std]

use {arch::wait_for_interrupt, core::panic::PanicInfo};

#[cfg(firmware = "uefi")]
use firmware::uefi;

#[cfg(any(firmware = "sbi", firmware = "tfa"))]
use core::arch::naked_asm;

#[cfg(any(firmware = "sbi", firmware = "tfa"))]
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

#[cfg(any(firmware = "sbi", firmware = "tfa"))]
#[unsafe(no_mangle)]
fn initialize_global(
    #[cfg(firmware = "sbi")] hartid: usize,
    #[cfg(firmware = "sbi")] device_tree: *const tree::Header,
) {
    main(unsafe {
        firmware::Global::new(
            #[cfg(firmware = "sbi")]
            hartid,
            #[cfg(firmware = "sbi")]
            device_tree,
        )
    });
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
    allocator::temporize();
    firmware::println!("Hello, firmware!");
    uart::println!("Hello, UART!");
    uart::dbg!(firmware::GLOBAL.lock().get_mut().unwrap());
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
