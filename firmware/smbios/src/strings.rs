use alloc::{string::String, vec::Vec};

/// # References
/// * [SMBIOS Specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.10.0.pdf) 6.1.3 Text strings
pub struct Strings<'a>(Bytes<'a>);

impl Strings<'_> {
    pub fn size(mut self) -> usize {
        while self.next().is_some() {}
        self.0.size
    }
}

impl<'a> From<&'a u8> for Strings<'a> {
    fn from(byte: &'a u8) -> Self {
        Self(Bytes { byte, size: 0 })
    }
}

impl<'a> Iterator for Strings<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut string: Vec<u8> = Vec::default();
        loop {
            match self.0.next().unwrap() {
                0x00 => break,
                byte => string.push(byte),
            }
        }
        (!string.is_empty()).then(|| String::from_utf8(string).unwrap())
    }
}

struct Bytes<'a> {
    byte: &'a u8,
    size: usize,
}

impl<'a> Iterator for Bytes<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let item: u8 = *self.byte;
        self.byte = unsafe { &*(self.byte as *const u8).add(1) };
        self.size += 1;
        Some(item)
    }
}
