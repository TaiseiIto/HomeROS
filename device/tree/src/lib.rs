#![feature(option_array_transpose)]
#![no_std]

use core::{
    fmt::{Debug, Formatter, Result},
    mem::size_of,
    slice::from_raw_parts,
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
        StructureIterator {
            header: self,
            structure_offset: 0,
        }
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

/// # References
/// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 5.4.1 Lexical structure
#[derive(Debug)]
enum Structure<'a> {
    BeginNode { name: &'a str },
    EndNode,
    Property { name: &'a str, data: &'a [u8] },
    Nop,
    End,
    Unknown { token: u32 },
}

#[derive(Clone)]
struct StructureIterator<'a> {
    header: &'a Header,
    structure_offset: usize,
}

impl StructureIterator<'_> {
    fn take_byte(&mut self) -> Option<u8> {
        let byte: Option<u8> = self
            .header
            .structure_bytes()
            .get(self.structure_offset)
            .copied();
        self.structure_offset += 1;
        byte
    }

    fn take_word(&mut self) -> Option<u32> {
        [
            self.take_byte(),
            self.take_byte(),
            self.take_byte(),
            self.take_byte(),
        ]
        .transpose()
        .map(u32::from_be_bytes)
    }
}

impl Debug for StructureIterator<'_> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter.debug_list().entries(self.clone()).finish()
    }
}

impl<'a> Iterator for StructureIterator<'a> {
    type Item = Structure<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.take_word().map(|token| match token {
            0x00000001 => {
                let Self {
                    header,
                    structure_offset,
                } = self;
                let remaining_bytes: &[u8] = &header.structure_bytes()[*structure_offset..];
                let name_size: usize = remaining_bytes
                    .iter()
                    .take_while(|byte| **byte != 0x00)
                    .enumerate()
                    .map(|(index, _)| index + 1)
                    .max()
                    .unwrap_or(0);
                let name: &str = str::from_utf8(&remaining_bytes[..name_size]).unwrap();
                *structure_offset += name_size + size_of::<u32>();
                *structure_offset &= !(size_of::<u32>() - 1);
                Self::Item::BeginNode { name }
            }
            0x00000002 => Self::Item::EndNode,
            0x00000003 => {
                let length: usize = self.take_word().unwrap() as usize;
                let name_offset: usize = self.take_word().unwrap() as usize;
                let Self {
                    header,
                    structure_offset,
                } = self;
                let name: &str = header.string(name_offset);
                let remaining_bytes: &[u8] = &header.structure_bytes()[*structure_offset..];
                let data: &[u8] = &remaining_bytes[..length];
                *structure_offset += length + size_of::<u32>() - 1;
                *structure_offset &= !(size_of::<u32>() - 1);
                Self::Item::Property { name, data }
            }
            0x00000004 => Self::Item::Nop,
            0x00000009 => Self::Item::End,
            token => Self::Item::Unknown { token },
        })
    }
}
