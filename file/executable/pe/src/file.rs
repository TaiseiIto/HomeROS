use core::fmt::{Debug, Formatter, Result};

/// # References
/// * [File Header (IMAGE_FILE_HEADER)](https://0xrick.github.io/win-internals/pe4/#file-header-image_file_header)
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

    fn machine(&self) -> Machine {
        self.machine.into()
    }
}

impl Debug for Header {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Header")
            .field("machine", &self.machine())
            .field("number_of_sections", &self.number_of_sections)
            .field("time_date_stamp", &self.time_date_stamp)
            .field("pointer_to_symbol_table", &self.pointer_to_symbol_table)
            .field("number_of_symbols", &self.number_of_symbols)
            .field("size_of_optional_header", &self.size_of_optional_header)
            .field("characteristics", &self.characteristics)
            .finish()
    }
}

#[derive(Debug)]
enum Machine {
    Unknown,
    Alpha,
    Alpha64Axp64,
    Am33,
    Amd64,
    Arm,
    Arm64,
    Arm64ec,
    Arm64x,
    Armnt,
    Ebc,
    I386,
    Ia64,
    LoongArch32,
    LoongArch64,
    M32r,
    Mips16,
    MipsFpu,
    MipsFpu16,
    PowerPc,
    PowerPcFp,
    R3000Be,
    R3000,
    R4000,
    R10000,
    Riscv32,
    Riscv64,
    Riscv128,
    Sh3,
    Sh3Dsp,
    Sh4,
    Sh5,
    Thumb,
    WceMipsV2,
}

impl From<u16> for Machine {
    fn from(machine: u16) -> Self {
        match machine {
            0x184 => Self::Alpha,
            0x284 => Self::Alpha64Axp64,
            0x1d3 => Self::Am33,
            0x8664 => Self::Amd64,
            0x1c0 => Self::Arm,
            0xaa64 => Self::Arm64,
            0xa641 => Self::Arm64ec,
            0xa64e => Self::Arm64x,
            0x1c4 => Self::Armnt,
            0xebc => Self::Ebc,
            0x14c => Self::I386,
            0x200 => Self::Ia64,
            0x6232 => Self::LoongArch32,
            0x6264 => Self::LoongArch64,
            0x9041 => Self::M32r,
            0x266 => Self::Mips16,
            0x366 => Self::MipsFpu,
            0x466 => Self::MipsFpu16,
            0x1f0 => Self::PowerPc,
            0x1f1 => Self::PowerPcFp,
            0x160 => Self::R3000Be,
            0x162 => Self::R3000,
            0x166 => Self::R4000,
            0x168 => Self::R10000,
            0x5032 => Self::Riscv32,
            0x5064 => Self::Riscv64,
            0x5128 => Self::Riscv128,
            0x1a2 => Self::Sh3,
            0x1a3 => Self::Sh3Dsp,
            0x1a6 => Self::Sh4,
            0x1a8 => Self::Sh5,
            0x1c2 => Self::Thumb,
            0x169 => Self::WceMipsV2,
            _ => Self::Unknown,
        }
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
