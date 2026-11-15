/// # References
/// * [File Header (IMAGE_FILE_HEADER)](https://0xrick.github.io/win-internals/pe4/#file-header-image_file_header)
#[derive(Debug)]
#[repr(C)]
pub struct Header {
    machine: Machine,
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

#[allow(dead_code)]
#[derive(Debug)]
#[repr(u16)]
enum Machine {
    Unknown = 0x0,
    Alpha = 0x184,
    Alpha64Axp64 = 0x284,
    Am33 = 0x1d3,
    Amd64 = 0x8664,
    Arm = 0x1c0,
    Arm64 = 0xaa64,
    Arm64ec = 0xa641,
    Arm64x = 0xa64e,
    Armnt = 0x1c4,
    Ebc = 0xebc,
    I386 = 0x14c,
    Ia64 = 0x200,
    LoongArch32 = 0x6232,
    LoongArch64 = 0x6264,
    M32r = 0x9041,
    Mips16 = 0x266,
    MipsFpu = 0x366,
    MipsFpu16 = 0x466,
    PowerPc = 0x1f0,
    PowerPcFp = 0x1f1,
    R3000Be = 0x160,
    R3000 = 0x162,
    R4000 = 0x166,
    R10000 = 0x168,
    Riscv32 = 0x5032,
    Riscv64 = 0x5064,
    Riscv128 = 0x5128,
    Sh3 = 0x1a2,
    Sh3Dsp = 0x1a3,
    Sh4 = 0x1a6,
    Sh5 = 0x1a8,
    Thumb = 0x1c2,
    WceMipsV2 = 0x169,
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
