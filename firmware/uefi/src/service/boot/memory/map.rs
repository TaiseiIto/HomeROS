use {
    super::{
        Type,
        address::{Physical, Virtual},
    },
    crate::Status,
    alloc::vec::Vec,
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [EFI_MEMORY_DESCRIPTOR](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-getmemorymap)
#[repr(C)]
pub struct Descriptor {
    memory_type: u32,
    physical_start: Physical,
    virtual_start: Virtual,
    number_of_pages: u64,
    attribute: AttributeRaw,
}

impl Descriptor {
    pub fn is_allocatable(&self) -> bool {
        matches!(self.memory_type.into(), Type::Conventional)
    }
}

impl Debug for Descriptor {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        let Self {
            memory_type,
            physical_start,
            virtual_start,
            number_of_pages,
            attribute,
        } = self;
        let memory_type: Type = (*memory_type).into();
        formatter
            .debug_struct("Descriptor")
            .field("memory_type", &memory_type)
            .field("physical_start", physical_start)
            .field("virtual_start", virtual_start)
            .field("number_of_pages", number_of_pages)
            .field("attribute", attribute)
            .finish()
    }
}

/// # References
/// * [Attribute](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-getmemorymap)
#[io::register]
pub struct Attribute {
    uc: bool,
    wc: bool,
    wt: bool,
    wb: bool,
    uce: bool,
    __: [bool; 7],
    wp: bool,
    rp: bool,
    xp: bool,
    nv: bool,
    more_reliable: bool,
    ro: bool,
    sp: bool,
    cpu_crypto: bool,
    hot_pluggable: bool,
    __: [bool; 23],
    isa_mask: u16,
    __: [bool; 2],
    isa_valid: bool,
    runtime: bool,
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
    pub fn iter<'a>(&'a self) -> Descriptors<'a> {
        Descriptors {
            map: self,
            index: 0,
        }
    }

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

pub struct Descriptors<'a> {
    map: &'a Map,
    index: usize,
}

impl<'a> Iterator for Descriptors<'a> {
    type Item = &'a Descriptor;

    fn next(&mut self) -> Option<Self::Item> {
        let Self {
            map:
                Map {
                    key: _,
                    descriptors,
                    descriptor_size,
                },
            index,
        } = self;
        descriptors
            .as_slice()
            .get(*index * descriptor_size)
            .map(|descriptor| {
                *index += 1;
                let descriptor: *const u8 = descriptor as *const u8;
                let descriptor: *const Descriptor = descriptor as *const Descriptor;
                unsafe { &*descriptor }
            })
    }
}
