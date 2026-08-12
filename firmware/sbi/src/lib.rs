#![no_std]

use core::arch::asm;

/// # References
/// * [sbiret](https://docs.riscv.org/reference/sbi/v3.0/binary-encoding.html)
#[derive(Debug)]
pub enum Error {
    Failed,
    NotSupported,
    InvalidParam,
    Denied,
    InvalidAddress,
    AlreadyAvailable,
    AlreadyStarted,
    AlreadyStopped,
    NoShmem,
    InvalidState,
    BadRange,
    Timeout,
    Io,
    DeniedLocked,
}

impl TryFrom<isize> for Error {
    type Error = ();

    fn try_from(result: isize) -> Result<Self, Self::Error> {
        match result {
            0 => Err(()),
            -1 => Ok(Self::Failed),
            -2 => Ok(Self::NotSupported),
            -3 => Ok(Self::InvalidParam),
            -4 => Ok(Self::Denied),
            -5 => Ok(Self::InvalidAddress),
            -6 => Ok(Self::AlreadyAvailable),
            -7 => Ok(Self::AlreadyStarted),
            -8 => Ok(Self::AlreadyStopped),
            -9 => Ok(Self::NoShmem),
            -10 => Ok(Self::InvalidState),
            -11 => Ok(Self::BadRange),
            -12 => Ok(Self::Timeout),
            -13 => Ok(Self::Io),
            -14 => Ok(Self::DeniedLocked),
            _ => unreachable!(),
        }
    }
}

/// # References
/// * [ECALL](https://docs.riscv.org/reference/sbi/v3.0/binary-encoding.html)
pub fn ecall(fid: i32, eid: i32, arguments: [usize; 6]) -> Result<usize, Error> {
    let [mut a0, mut a1, a2, a3, a4, a5]: [usize; 6] = arguments;
    unsafe {
        asm!(
            "ecall",
            inout("a0") a0,
            inout("a1") a1,
            in("a2") a2,
            in("a3") a3,
            in("a4") a4,
            in("a5") a5,
            in("a6") fid,
            in("a7") eid,
        );
    }
    let error: Result<Error, ()> = A0Error::a0(a0).error().try_into();
    match error {
        Ok(error) => Err(error),
        Err(()) => Ok(a1),
    }
}

union A0Error {
    a0: usize,
    error: isize,
}

impl A0Error {
    fn a0(a0: usize) -> Self {
        Self { a0 }
    }

    fn error(self) -> isize {
        unsafe { self.error }
    }
}
