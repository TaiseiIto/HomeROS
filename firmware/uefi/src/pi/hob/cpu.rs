use super::Header;

/// # References
/// * [EFI_HOB_CPU](https://uefi.org/specs/PI/1.10/V3_HOB_Code_Definitions.html#efi-hob-cpu)
#[derive(Debug)]
#[repr(C)]
pub struct Hob {
    header: Header,
    size_of_memory_space: u8,
    size_of_io_space: u8,
    reserved: [u8; 6],
}

impl super::Hob for Hob {}
