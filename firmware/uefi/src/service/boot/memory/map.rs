use {
    super::address::{Physical, Virtual},
    crate::Status,
    alloc::vec::Vec,
};

/// # References
/// * [EFI_MEMORY_DESCRIPTOR](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-getmemorymap)
#[repr(C)]
pub struct Descriptor {
    memory_type: u32,
    physical_start: Physical,
    virtual_start: Virtual,
    number_of_pages: u64,
    attribute: u64,
}

/// Refeernces
/// * [GetMemoryMap](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-getmemorymap)
pub type Get =
    extern "efiapi" fn(*mut usize, *mut Descriptor, *mut usize, *mut usize, *mut u32) -> Status;

#[derive(Debug)]
pub struct Map {
    key: usize,
    descriptors: Vec<u8>,
    descriptor_size: usize,
}

impl Map {
    pub fn key(&self) -> usize {
        self.key
    }

    pub fn new(key: usize, descriptors: Vec<u8>, descriptor_size: usize) -> Self {
        Self {
            key,
            descriptors,
            descriptor_size,
        }
    }
}
