use {super::Header, crate::service::boot::memory::address::Physical};

/// # References
/// * [EFI_HOB_HANDOFF_INFO_TABLE](https://uefi.org/specs/PI/1.10/V3_HOB_Code_Definitions.html#efi-hob-handoff-info-table-phit-hob)
#[derive(Debug)]
#[repr(C)]
pub struct Hob {
    header: Header,
    version: u32,
    boot_mode: BootMode,
    memory_top: Physical,
    memory_bottom: Physical,
    free_memory_top: Physical,
    free_memory_bottom: Physical,
    end_of_hob_list: Physical,
}

impl super::Hob for Hob {}

type BootMode = u32;
