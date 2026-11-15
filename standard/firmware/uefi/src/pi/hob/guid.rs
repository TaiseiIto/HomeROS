use {super::Header, crate::Guid};

/// # References
/// * [EFI_HOB_GUID_TYPE](https://uefi.org/specs/PI/1.10/V3_HOB_Code_Definitions.html#efi-hob-guid-type)
#[derive(Debug)]
#[repr(C)]
pub struct Hob {
    header: Header,
    name: Guid,
}

impl super::Hob for Hob {}
