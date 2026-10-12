use {
    alloc::string::String,
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [Section Table (Section Headers)](https://learn.microsoft.com/en-us/windows/win32/debug/pe-format#section-table-section-headers)
#[repr(C)]
pub struct Header {
    name: [u8; 8],
    virtual_size: u32,
    virtual_address: u32,
    size_of_raw_data: u32,
    pointer_to_raw_data: u32,
    pointer_to_relocations: u32,
    pointer_to_linenumbers: u32,
    number_of_relocations: u16,
    number_of_linenumbers: u16,
    characteristics: u32,
}

impl Header {
    fn name(&self) -> String {
        str::from_utf8(self.name.as_slice())
            .unwrap()
            .chars()
            .take_while(|character| *character != '\0')
            .collect()
    }
}

impl Debug for Header {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Header")
            .field("name", &self.name())
            .field("virtual_size", &self.virtual_size)
            .field("virtual_address", &self.virtual_address)
            .field("size_of_raw_data", &self.size_of_raw_data)
            .field("pointer_to_raw_data", &self.pointer_to_raw_data)
            .field("pointer_to_relocations", &self.pointer_to_relocations)
            .field("pointer_to_linenumbers", &self.pointer_to_linenumbers)
            .field("number_of_relocations", &self.number_of_relocations)
            .field("number_of_linenumbers", &self.number_of_linenumbers)
            .field("characteristics", &self.characteristics)
            .finish()
    }
}
