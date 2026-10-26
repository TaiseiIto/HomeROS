use {super::Header, crate::service::boot::memory::address::Physical};

/// # References
/// * [EFI_HOB_UEFI_CAPSULE](https://uefi.org/specs/PI/1.10/V3_HOB_Code_Definitions.html#efi-hob-uefi-capsule)
#[derive(Debug)]
#[repr(C)]
pub struct Hob {
    header: Header,
    base_address: Physical,
    length: u64,
}

impl super::Hob for Hob {}
