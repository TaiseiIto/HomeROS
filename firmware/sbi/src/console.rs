use crate::{Error, ecall};

/// # References
/// * [Console Putchar](https://docs.riscv.org/reference/sbi/v3.0/ext-legacy.html#5-1-2-extension-console-putchar-eid-0x01)
pub fn putchar(byte: u8) {
    let fid: i32 = 0;
    let eid: i32 = 1;
    let a0: usize = byte as usize;
    let a1: usize = 0;
    let a2: usize = 0;
    let a3: usize = 0;
    let a4: usize = 0;
    let a5: usize = 0;
    ecall(fid, eid, [a0, a1, a2, a3, a4, a5]).unwrap();
}
