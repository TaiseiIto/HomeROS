use core::arch::asm;

#[inline(always)]
pub unsafe fn wait_for_interrupt() {
    unsafe {
        asm!("hlt");
    }
}
