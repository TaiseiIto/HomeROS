use {
    super::{Header, Hob},
    crate::{Guid, service::boot::memory::address::Physical},
};

/// # References
/// * [EFI_HOB_FIRMWARE_VOLUME](https://uefi.org/specs/PI/1.10/V3_HOB_Code_Definitions.html#efi-hob-firmware-volume)
#[derive(Debug)]
#[repr(C)]
pub struct First {
    header: Header,
    base_address: Physical,
    length: u64,
}

impl Hob for First {}

/// # References
/// * [EFI_HOB_FIRMWARE_VOLUME2](https://uefi.org/specs/PI/1.10/V3_HOB_Code_Definitions.html#efi-hob-firmware-volume2)
#[derive(Debug)]
#[repr(C)]
pub struct Second {
    header: Header,
    base_address: Physical,
    length: u64,
    fv_name: Guid,
    file_name: Guid,
}

impl Hob for Second {}

/// # References
/// * [EFI_HOB_FIRMWARE_VOLUME3](https://uefi.org/specs/PI/1.10/V3_HOB_Code_Definitions.html#efi-hob-firmware-volume3)
#[derive(Debug)]
#[repr(C)]
pub struct Third {
    header: Header,
    base_address: Physical,
    length: u64,
    authentication_status: u32,
    extracted_fv: bool,
    fv_name: Guid,
    file_name: Guid,
}

impl Hob for Third {}
