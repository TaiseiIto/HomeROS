#![no_std]

#[cfg(target_arch = "x86_64")]
extern crate alloc;

#[cfg(target_arch = "x86_64")]
mod x64;

pub use x64::*;
