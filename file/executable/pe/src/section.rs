/// # References
/// * [Section Table (Section Headers)](https://learn.microsoft.com/en-us/windows/win32/debug/pe-format#section-table-section-headers)
#[derive(Debug)]
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
