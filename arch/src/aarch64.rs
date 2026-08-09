use core::arch::asm;

/// # Safety
/// This function stops CPU until the next interruption.
/// # References
/// * [WFI](https://support.arm.com/documentation/ddi0487/mc/-Part-C-The-AArch64-Instruction-Set/-Chapter-C6-A64-Base-Instruction-Descriptions/-C6-2-Alphabetical-list-of-A64-base-instructions/-C6-2-504-WFI?lang=en)
#[inline(always)]
pub unsafe fn wait_for_interrupt() {
    unsafe {
        asm!("wfi");
    }
}
