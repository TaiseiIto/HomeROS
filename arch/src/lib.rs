#![no_std]

use core::arch::asm;

/// # References
/// * [AArch64](https://support.arm.com/documentation/ddi0487/mc/-Part-C-The-AArch64-Instruction-Set/-Chapter-C6-A64-Base-Instruction-Descriptions/-C6-2-Alphabetical-list-of-A64-base-instructions/-C6-2-508-YIELD?lang=en)
/// * [RISC-V64](https://docs.riscv.org/reference/isa/v20260120/unpriv/zihintpause.html)
/// * [x86-64](https://cdrdv2.intel.com/v1/dl/getContent/671200) p.1515
#[inline(always)]
pub fn pause() {
    unsafe {
        #[cfg(target_arch = "aarch64")]
        asm!("yield");
        #[cfg(target_arch = "riscv64")]
        asm!("pause");
        #[cfg(target_arch = "x86_64")]
        asm!("pause");
    }
}

/// # Safety
/// This function stops CPU until the next interruption.
/// # References
/// * [AArch64](https://support.arm.com/documentation/ddi0487/mc/-Part-C-The-AArch64-Instruction-Set/-Chapter-C6-A64-Base-Instruction-Descriptions/-C6-2-Alphabetical-list-of-A64-base-instructions/-C6-2-504-WFI?lang=en)
/// * [RISC-V64](https://docs.riscv.org/reference/isa/v20260120/priv/priv-insns.html)
/// * [x86-64](https://cdrdv2.intel.com/v1/dl/getContent/671200) p.1143
#[inline(always)]
pub unsafe fn wait_for_interrupt() {
    unsafe {
        #[cfg(target_arch = "aarch64")]
        asm!("wfi");
        #[cfg(target_arch = "riscv64")]
        asm!("wfi");
        #[cfg(target_arch = "x86_64")]
        asm!("hlt");
    }
}
