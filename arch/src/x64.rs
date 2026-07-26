use core::arch::asm;

/// # Safety
/// This function stops CPU until the next interruption.
#[inline(always)]
pub unsafe fn wait_for_interrupt() {
    unsafe {
        asm!("hlt");
    }
}
