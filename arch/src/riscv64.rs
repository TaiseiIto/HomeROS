use core::arch::asm;

/// # Safety
/// This function stops CPU until the next interruption.
/// # References
/// * [RISC-V Privileged Instruction Set Listings](https://docs.riscv.org/reference/isa/v20260120/priv/priv-insns.html)
#[inline(always)]
pub unsafe fn wait_for_interrupt() {
    unsafe {
        asm!("wfi");
    }
}
