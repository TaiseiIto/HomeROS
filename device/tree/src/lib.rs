#![feature(iter_array_chunks)]
#![no_std]

extern crate alloc;

mod property;
mod structure;

use {
    core::{
        fmt::{Debug, Formatter, Result},
        slice::from_raw_parts,
    },
    structure::StructureIterator,
};

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

impl Header {
    fn iter(&self) -> StructureIterator<'_> {
        self.into()
    }

    fn string(&self, offset: usize) -> &str {
        let strings_bytes: &[u8] = self.strings_bytes();
        let strings_bytes_len: usize = strings_bytes.len();
        let end: usize = (offset..strings_bytes_len)
            .take_while(|offset| strings_bytes.get(*offset).is_some_and(|byte| *byte != 0x00))
            .max()
            .map(|last_index| last_index + 1)
            .unwrap_or(offset);
        str::from_utf8(&strings_bytes[offset..end]).unwrap()
    }

    fn strings_bytes(&self) -> &[u8] {
        let offset: usize = self.read_off_dt_strings() as usize;
        let size: usize = self.read_size_dt_strings() as usize;
        let header: *const Self = self as *const Self;
        let header: *const u8 = header as *const u8;
        unsafe { from_raw_parts(header.add(offset), size) }
    }

    fn structure_bytes(&self) -> &[u8] {
        let offset: usize = self.read_off_dt_struct() as usize;
        let size: usize = self.read_size_dt_struct() as usize;
        let header: *const Self = self as *const Self;
        let header: *const u8 = header as *const u8;
        unsafe { from_raw_parts(header.add(offset), size) }
    }
}

impl Debug for Header {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Header")
            .field("magic", &self.read_magic())
            .field("totalsize", &self.read_totalsize())
            .field("structures", &self.iter())
            .field("off_mem_rsvmap", &self.read_off_mem_rsvmap())
            .field("version", &self.read_version())
            .field("last_comp_version", &self.read_last_comp_version())
            .field("boot_cpuid_phys", &self.read_boot_cpuid_phys())
            .finish()
    }
}
