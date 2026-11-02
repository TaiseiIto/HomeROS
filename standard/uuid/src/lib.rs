#![no_std]

use core::fmt::{self, Debug, Formatter};

#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct Uuid {
    data1: u32,
    data2: u16,
    data3: u16,
    data4: [u8; 8],
}

impl Uuid {
    pub const fn new(data1: u32, data2: u16, data3: u16, data4: [u8; 8]) -> Self {
        Self {
            data1,
            data2,
            data3,
            data4,
        }
    }
}

impl Debug for Uuid {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_fmt(format_args!(
            "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
            self.data1,
            self.data2,
            self.data3,
            self.data4[..2]
                .iter()
                .fold(0u16, |data, byte| (data << u8::BITS) + (*byte as u16)),
            self.data4[2..]
                .iter()
                .fold(0u64, |data, byte| (data << u8::BITS) + (*byte as u64)),
        ))
    }
}
