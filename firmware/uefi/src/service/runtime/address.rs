use {
    super::super::boot::memory::map::Descriptor,
    crate::{Status, Void},
};

/// References
/// * [ConvertPointer](https://uefi.org/specs/UEFI/2.11/08_Services_Runtime_Services.html#convertpointer)
#[must_use]
pub type Get = extern "efiapi" fn(usize, *mut *const Void) -> Status;

/// References
/// * [SetVirtualAddressMap](https://uefi.org/specs/UEFI/2.11/08_Services_Runtime_Services.html#setvirtualaddressmap)
#[must_use]
pub type Set = extern "efiapi" fn(usize, usize, u32, Descriptor) -> Status;
