/// # References
/// * [File Header (IMAGE_FILE_HEADER)](https://0xrick.github.io/win-internals/pe4/#file-header-image_file_header)
#[derive(Debug)]
#[repr(C)]
pub struct Header {
    machine: u16,
    number_of_sections: u16,
    time_date_stamp: u32,
    pointer_to_symbol_table: u32,
    number_of_symbols: u32,
    size_of_optional_header: u16,
    characteristics: u16,
}
