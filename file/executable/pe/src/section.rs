use {
    alloc::string::String,
    core::{
        fmt::{Debug, Formatter, Result},
        ops::Range,
    },
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
    characteristics: FlagsRaw,
}

impl Header {
    fn name(&self) -> String {
        str::from_utf8(self.name.as_slice())
            .unwrap()
            .chars()
            .take_while(|character| *character != '\0')
            .collect()
    }

    fn range_on_file(&self) -> Range<usize> {
        let start_on_file: usize = self.pointer_to_raw_data as usize;
        let size_on_file: usize = self.size_of_raw_data as usize;
        let end_on_file: usize = start_on_file + size_on_file;
        start_on_file..end_on_file
    }

    fn range_on_memory(&self) -> Range<usize> {
        let start_on_memory: usize = self.virtual_address as usize;
        let size_on_memory: usize = self.virtual_size as usize;
        let end_on_memory: usize = start_on_memory + size_on_memory;
        start_on_memory..end_on_memory
    }
}

impl Debug for Header {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Header")
            .field("name", &self.name())
            .field("range_on_memory", &self.range_on_memory())
            .field("range_on_file", &self.range_on_file())
            .field("pointer_to_relocations", &self.pointer_to_relocations)
            .field("pointer_to_linenumbers", &self.pointer_to_linenumbers)
            .field("number_of_relocations", &self.number_of_relocations)
            .field("number_of_linenumbers", &self.number_of_linenumbers)
            .field("characteristics", &self.characteristics)
            .finish()
    }
}

/// # References
/// * [Section Flags](https://learn.microsoft.com/en-us/windows/win32/debug/pe-format#section-flags)
#[io::register]
struct Flags {
    __: [bool; 3],
    type_no_pad: bool,
    __: bool,
    cnt_code: bool,
    cnt_initialized_data: bool,
    cnt_uninitialized_data: bool,
    lnk_other: bool,
    lnk_info: bool,
    __: bool,
    lnk_remove: bool,
    lnk_comdat: bool,
    __: [bool; 2],
    scn_gprel: bool,
    __: [bool; 4],
    align: [bool; 4],
    lnk_nreloc_ovfl: bool,
    mem_discardable: bool,
    mem_not_cached: bool,
    mem_not_paged: bool,
    mem_shared: bool,
    mem_execute: bool,
    mem_read: bool,
    mem_write: bool,
}
