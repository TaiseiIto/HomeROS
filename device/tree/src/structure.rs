use {
    crate::{header::Header, property::Property},
    alloc::{
        string::{String, ToString},
        vec::Vec,
    },
    core::{
        fmt::{Debug, Formatter, Result},
        mem::size_of,
    },
};

/// # References
/// * [Devicetree Specification](https://github.com/devicetree-org/devicetree-specification/releases/download/v0.4/devicetree-specification-v0.4.pdf) 5.4.1 Lexical structure
#[derive(Debug)]
pub enum Structure {
    BeginNode { name: String },
    End,
    EndNode,
    Nop,
    Property(Property),
}

#[derive(Clone)]
pub struct StructureIterator<'a> {
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
        (0..size_of::<u32>())
            .map(|_| self.take_byte())
            .collect::<Option<Vec<u8>>>()
            .map::<[u8; size_of::<u32>()], _>(|word| word.try_into().unwrap())
            .map(u32::from_be_bytes)
    }
}

impl Debug for StructureIterator<'_> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter.debug_list().entries(self.clone()).finish()
    }
}

impl<'a> From<&'a Header> for StructureIterator<'a> {
    fn from(header: &'a Header) -> Self {
        Self {
            header,
            structure_offset: 0,
        }
    }
}

impl Iterator for StructureIterator<'_> {
    type Item = Structure;

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
                Self::Item::Property(Property::new(name, data))
            }
            0x00000004 => Self::Item::Nop,
            0x00000009 => Self::Item::End,
            _ => panic!(),
        })
    }
}
