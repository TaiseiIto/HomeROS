#![no_main]
#![no_std]

#[cfg(target_arch = "riscv64")]
#[link_section = ".text.main"]
#[naked]
#[unsafe(no_mangle)]
unsafe extern "C" fn _start() -> ! {
    asm!("la sp, _stack_top", "j main", options(noreturn));
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
fn panic(_panic: &core::panic::PanicInfo) -> ! {
    loop {}
}
