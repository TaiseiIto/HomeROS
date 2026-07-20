#![no_main]
#![no_std]

#[cfg(firmware = "open_sbi")]
#[no_mangle]
pub extern "C" fn open_sbi_main() {
    main();
}

#[cfg(firmware = "uefi")]
#[unsafe(no_mangle)]
fn efi_main() {
    main();
}

fn main() {
    unimplemented!();
}

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo) -> ! {
    loop {}
}
