pub mod address;

/// References
/// * [EFI_MEMORY_DESCRIPTOR](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-getmemorymap)
#[repr(C)]
pub struct Descriptor {
    memory_type: u32,
    physical_start: address::Physical,
    virtual_start: address::Virtual,
    number_of_pages: u64,
    attribute: u64,
}
