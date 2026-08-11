use {
    super::{super::boot::memory::address::Physical, ResetType},
    crate::{Guid, Status},
};

/// # References
/// * [QueryCapsuleCapabilities](https://uefi.org/specs/UEFI/2.11/08_Services_Runtime_Services.html#querycapsulecapabilities)
#[must_use]
pub type QueryCapabilities =
    extern "efiapi" fn(*const *const Header, usize, *mut u64, *mut ResetType) -> Status;

/// # References
/// * [UpdateCapsule](https://uefi.org/specs/UEFI/2.11/08_Services_Runtime_Services.html#updatecapsule)
#[must_use]
pub type Update = extern "efiapi" fn(*const *const Header, usize, Physical) -> Status;

/// # References
/// * [EFI_CAPSULE_HEADER](https://uefi.org/specs/UEFI/2.11/08_Services_Runtime_Services.html#updatecapsule)
#[repr(C)]
pub struct Header {
    guild: Guid,
    header_size: u32,
    flags: u32,
    image_size: u32,
}
