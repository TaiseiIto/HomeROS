#![no_std]

use core::arch::asm;

/// # Safety
/// This function stops CPU until the next interruption.
/// # References
/// * [WFI](https://support.arm.com/documentation/ddi0487/mc/-Part-C-The-AArch64-Instruction-Set/-Chapter-C6-A64-Base-Instruction-Descriptions/-C6-2-Alphabetical-list-of-A64-base-instructions/-C6-2-504-WFI?lang=en)
/// * [RISC-V Privileged Instruction Set Listings](https://docs.riscv.org/reference/isa/v20260120/priv/priv-insns.html)
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual June 2026](https://cdrdv2.intel.com/v1/dl/getContent/671200) p.1143
#[inline(always)]
pub unsafe fn wait_for_interrupt() {
    unsafe {
        asm!(
            #[cfg(target_arch = "aarch64")]
            "wfi",
            #[cfg(target_arch = "riscv64")]
            "wfi",
            #[cfg(target_arch = "x86_64")]
            "hlt",
        );
    }
}
