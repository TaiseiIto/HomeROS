#![no_main]
#![no_std]

#[unsafe(no_mangle)]
fn efi_main() {
    unimplemented!();
}

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo) -> ! {
    loop {}
}
