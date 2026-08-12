#![no_std]

use core::arch::asm;

/// # References
/// * [sbiret](https://docs.riscv.org/reference/sbi/v3.0/binary-encoding.html)
pub struct Ret {
    error: usize,
    value: usize,
}

/// # References
/// * [ECALL](https://docs.riscv.org/reference/sbi/v3.0/binary-encoding.html)
pub fn ecall(
    fid: usize,
    eid: usize,
    a0: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
) -> Ret {
    let mut error: usize = a0;
    let mut value: usize = a1;
    unsafe {
        asm!(
            "ecall",
            inout("a0") error,
            inout("a1") value,
            in("a2") a2,
            in("a3") a3,
            in("a4") a4,
            in("a5") a5,
            in("a6") fid,
            in("a7") eid,
        );
    }
    Ret { error, value }
}
