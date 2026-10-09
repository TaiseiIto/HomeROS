#![no_main]
#![no_std]

extern crate alloc;

use {arch::wait_for_interrupt, core::panic::PanicInfo};

#[cfg(firmware = "uefi")]
use firmware::uefi::{HandleMut, Status, system::Table};

#[cfg(start_with_assembly)]
use core::arch::naked_asm;

#[cfg(start_with_assembly)]
#[unsafe(link_section = ".text._start")]
#[unsafe(naked)]
#[unsafe(no_mangle)]
unsafe extern "C" fn _start() -> ! {
    #[cfg(target_arch = "aarch64")]
    naked_asm!(
        "ldr x0, =_stack_bottom",
        "mov sp, x0",
        "ldr x0, =.text._start",
        "ldr x1, =_heap_head",
        "b initialize_global"
    );
    #[cfg(target_arch = "riscv64")]
    naked_asm!(
        "la sp, _stack_bottom",
        "la a2, .text._start",
        "la a3, _heap_head",
        "j initialize_global"
    );
}

#[cfg(start_with_assembly)]
#[unsafe(no_mangle)]
fn initialize_global(
    #[cfg(firmware = "sbi")] hartid: usize,
    #[cfg(firmware = "sbi")] device_tree: *const tree::Header,
    boot_loader_head: usize,
    heap_head: usize,
) {
    main(unsafe {
        firmware::Global::new(
            #[cfg(firmware = "sbi")]
            hartid,
            #[cfg(firmware = "sbi")]
            device_tree,
            boot_loader_head,
            heap_head,
        )
    });
    unreachable!();
}

/// # References
/// * [EFI_IMAGE_ENTRY_POINT](https://uefi.org/specs/UEFI/2.11/04_EFI_System_Table.html#efi-image-entry-point)
#[cfg(firmware = "uefi")]
#[unsafe(no_mangle)]
extern "efiapi" fn efi_main(image_handle: HandleMut, system_table: *mut Table) -> Status {
    main(unsafe { firmware::Global::new(image_handle, system_table) });
    unreachable!();
}

fn main(global: firmware::Global) {
    global.set();
    allocator::temporize(
        #[cfg(use_temporary_memory_allocator)]
        firmware::GLOBAL.lock().get().unwrap().boot_heap_head(),
    );
    #[cfg(has_device_tree)]
    tree::set(firmware::GLOBAL.lock().get().unwrap().device_tree());
    uart::initialize(
        #[cfg(has_device_tree)]
        tree::ROOT
            .lock()
            .get()
            .unwrap()
            .uarts()
            .into_iter()
            .min_by_key(|uart| uart.base_address())
            .unwrap(),
    );
    firmware::println!("Hello, firmware!");
    uart::println!("Hello, UART!");
    #[cfg(has_device_tree)]
    uart::dbg!(tree::ROOT.lock().get().unwrap());
    allocator::stabilize(
        #[cfg(firmware = "uefi")]
        firmware::GLOBAL
            .lock()
            .get_mut()
            .unwrap()
            .exit_boot_services(),
        #[cfg(has_device_tree)]
        tree::memory_regions().try_cast().unwrap(),
        #[cfg(start_with_assembly)]
        firmware::GLOBAL.lock().get().unwrap().boot_loader_head(),
    );
    uart::dbg!(firmware::GLOBAL.lock().get_mut().unwrap());
    #[cfg(firmware = "uefi")]
    uart::dbg!(firmware::GLOBAL.lock().get().unwrap().system_table().rsdp());
    unimplemented!();
}

#[panic_handler]
fn panic(panic: &PanicInfo) -> ! {
    uart::println!("{}", panic);
    loop {
        unsafe {
            wait_for_interrupt();
        }
    }
}
