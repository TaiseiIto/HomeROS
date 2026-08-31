#![feature(option_array_transpose)]
#![no_std]

extern crate alloc;

use {
    alloc::string::{String, ToString},
    core::{
        fmt::{Debug, Formatter, Result},
        mem::size_of,
        slice::from_raw_parts,
    },
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

#[derive(Clone)]
struct CompatibleStrings<'a> {
    strings: &'a [u8],
    offset: usize,
}

impl<'a> CompatibleStrings<'a> {
    fn new(strings: &'a [u8]) -> Self {
        Self { strings, offset: 0 }
    }
}

impl Debug for CompatibleStrings<'_> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter.debug_list().entries(self.clone()).finish()
    }
}

impl<'a> Iterator for CompatibleStrings<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let Self { strings, offset } = self;
        let strings_length: usize = strings.len();
        let begin: usize = *offset;
        if begin < strings_length {
            let end: usize = (begin..strings_length)
                .take_while(|offset| strings.get(*offset).is_some_and(|byte| *byte != 0x00))
                .max()
                .map(|last_index| last_index + 1)
                .unwrap_or(begin);
            *offset += end - begin + 1;
            Some(str::from_utf8(&strings[begin..end]).unwrap())
        } else {
            None
        }
    }
}

/// # References
/// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 2.3 Standard Properties
#[derive(Debug)]
enum Property<'a> {
    AddressCells(u32),
    Compatible(&'a [u8]),
    Model(&'a str),
    SizeCells(u32),
    Unknown { name: &'a str, data: &'a [u8] },
}

impl Property<'_> {
    fn data2u32(data: &[u8]) -> u32 {
        [
            data.get(0).copied(),
            data.get(1).copied(),
            data.get(2).copied(),
            data.get(3).copied(),
        ]
        .transpose()
        .map(u32::from_be_bytes)
        .unwrap()
    }

    fn data2str(data: &[u8]) -> &str {
        str::from_utf8(&data[..data.len() - 1]).unwrap()
    }
}

impl<'a> Property<'a> {
    fn new(name: &'a str, data: &'a [u8]) -> Self {
        match name {
            "#address-cells" => Self::AddressCells(Self::data2u32(data)),
            "#size-cells" => Self::SizeCells(Self::data2u32(data)),
            "compatible" => Self::Compatible(data),
            "model" => Self::Model(Self::data2str(data)),
            name => Self::Unknown { name, data },
        }
    }
}

/// # References
/// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 5.4.1 Lexical structure
enum Structure<'a> {
    BeginNode { name: String },
    EndNode,
    Property { name: &'a str, data: &'a [u8] },
    Nop,
    End,
    Unknown { token: u32 },
}

impl Debug for Structure<'_> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::BeginNode { name } => formatter
                .debug_struct("BeginNode")
                .field("name", &name)
                .finish(),
            Self::EndNode => formatter.debug_struct("EndNode").finish(),
            Self::Property { name, data } => match Property::new(name, data) {
                Property::AddressCells(cells) => formatter
                    .debug_tuple("Property::AddressCells")
                    .field(&cells)
                    .finish(),
                Property::Compatible(strings) => formatter
                    .debug_struct("Property::Compatible")
                    .field("strings", &CompatibleStrings::new(strings))
                    .finish(),
                Property::Model(string) => formatter
                    .debug_tuple("Property::Model")
                    .field(&string)
                    .finish(),
                Property::SizeCells(cells) => formatter
                    .debug_tuple("Property::SizeCells")
                    .field(&cells)
                    .finish(),
                Property::Unknown { name, data } => formatter
                    .debug_struct("Property::Unknown")
                    .field("name", &name)
                    .field("data", &data)
                    .finish(),
            },
            Self::Nop => formatter.debug_struct("Nop").finish(),
            Self::End => formatter.debug_struct("End").finish(),
            Self::Unknown { token } => formatter
                .debug_struct("Unknown")
                .field("token", &token)
                .finish(),
        }
    }
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
                let name: String = str::from_utf8(&remaining_bytes[..name_size])
                    .unwrap()
                    .to_string();
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
