#![no_std]

use core::fmt::{Debug, Formatter, Result};

/// # References
/// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 5.2 Header
#[derive(endian::Big)]
#[repr(C)]
pub struct Header {
    magic: u32,
    totalsize: u32,
    off_dt_struct: u32,
    off_dt_strings: u32,
    off_mem_rsvmap: u32,
    version: u32,
    last_comp_version: u32,
    boot_cpuid_phys: u32,
    size_dt_strings: u32,
    size_dt_struct: u32,
}

impl Debug for Header {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Header")
            .field("magic", &self.read_magic())
            .field("totalsize", &self.read_totalsize())
            .field("off_dt_struct", &self.read_off_dt_struct())
            .field("off_dt_strings", &self.read_off_dt_strings())
            .field("off_mem_rsvmap", &self.read_off_mem_rsvmap())
            .field("version", &self.read_version())
            .field("last_comp_version", &self.read_last_comp_version())
            .field("boot_cpuid_phys", &self.read_boot_cpuid_phys())
            .field("size_dt_strings", &self.read_size_dt_strings())
            .field("size_dt_struct", &self.read_size_dt_struct())
            .finish()
    }
}
