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
    characteristics: CharacteristicsRaw,
}

impl Header {
    pub fn number_of_sections(&self) -> usize {
        self.number_of_sections as usize
    }

    pub fn size_of_optional_header(&self) -> usize {
        self.size_of_optional_header as usize
    }
}

#[io::register]
struct Characteristics {
    relocs_stripped: bool,
    executable_image: bool,
    line_nums_stripped: bool,
    local_symc_stripped: bool,
    aggressive_ws_trim: bool,
    large_address_aware: bool,
    __: bool,
    bytes_reserved_lo: bool,
    bit32_machine: bool,
    debug_stripped: bool,
    removable_run_from_swap: bool,
    net_run_from_swap: bool,
    system: bool,
    dll: bool,
    up_system_only: bool,
    bytes_reserved_hi: bool,
}
