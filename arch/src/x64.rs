use core::arch::asm;

/// # Safety
/// This function stops CPU until the next interruption.
/// # References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual June 2026](https://cdrdv2.intel.com/v1/dl/getContent/671200) p.1143
#[inline(always)]
pub unsafe fn wait_for_interrupt() {
    unsafe {
        asm!("hlt");
    }
}
