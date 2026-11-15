use super::super::Header;

/// # References
/// * [EFI_HOB_MEMORY_POOL](https://uefi.org/specs/PI/1.10/V3_HOB_Code_Definitions.html#efi-hob-memory-pool)
#[derive(Debug)]
#[repr(C)]
pub struct Hob {
    header: Header,
}

impl super::super::Hob for Hob {}
