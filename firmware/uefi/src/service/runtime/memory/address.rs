use {
    super::Descriptor,
    crate::{Status, Void},
};

/// References
/// * [ConvertPointer](https://uefi.org/specs/UEFI/2.11/08_Services_Runtime_Services.html#convertpointer)
pub type Get = extern "efiapi" fn(usize, *mut *const Void) -> Status;

/// References
/// * [EFI_PHYSICAL_ADDRESS](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-allocatepages)
pub type Physical = u64;

/// References
/// * [SetVirtualAddressMap](https://uefi.org/specs/UEFI/2.11/08_Services_Runtime_Services.html#setvirtualaddressmap)
pub type Set = extern "efiapi" fn(usize, usize, u32, Descriptor) -> Status;

/// References
/// * [EFI_VIRTUAL_ADDRESS](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-getmemorymap)
pub type Virtual = u64;
