use {
    super::boot::memory::address::Physical,
    crate::{Handle, Status, table},
};

/// # References
/// * [DXE_SERVICES](https://uefi.org/specs/PI/1.8/V2_UEFI_System_Table.html#dxe-services)
#[derive(Debug)]
#[repr(C)]
pub struct Table {
    hdr: table::Header,
    add_memory_space: Option<AddMemorySpace>,
    allocate_memory_space: Option<AllocateMemorySpace>,
    free_memory_space: Option<FreeMemorySpace>,
    remove_memory_space: Option<RemoveMemorySpace>,
    get_memory_space_descriptor: Option<GetMemorySpaceDescriptor>,
}

/// # References
/// * [EFI_ADD_MEMORY_SPACE](https://uefi.org/specs/PI/1.8/V2_Services_DXE_Services.html#addmemoryspace)
pub type AddMemorySpace = extern "efiapi" fn(GcdMemoryType, Physical, u64, u64) -> Status;

/// # References
/// * [EFI_GCD_MEMORY_TYPE](https://uefi.org/specs/PI/1.8/V2_Services_DXE_Services.html#addmemoryspace)
#[derive(Debug)]
#[repr(C)]
pub enum GcdMemoryType {
    NonExistent,
    Reserved,
    SystemMemory,
    MemoryMappedIo,
    Persistent,
    MoreReliable,
    Unaccepted,
    Maximum,
}

/// # References
/// * [EFI_ALLOCATE_MEMORY_SPACE](https://uefi.org/specs/PI/1.8/V2_Services_DXE_Services.html#allocatememoryspace)
pub type AllocateMemorySpace = extern "efiapi" fn(
    GcdAllocateType,
    GcdMemoryType,
    usize,
    u64,
    *mut Physical,
    Handle,
    Handle,
) -> Status;

/// # References
/// * [EFI_GCD_ALLOCATE_TYPE](https://uefi.org/specs/PI/1.8/V2_Services_DXE_Services.html#allocatememoryspace)
#[derive(Debug)]
#[repr(C)]
pub enum GcdAllocateType {
    AnySearchBottomUp,
    MaxAddressSearchBottomUp,
    Address,
    AnySearchTopDown,
    MaxAddressSearchTopDown,
    MaxType,
}

/// # References
/// * [EFI_FREE_MEMORY_SPACE](https://uefi.org/specs/PI/1.8/V2_Services_DXE_Services.html#freememoryspace)
pub type FreeMemorySpace = extern "efiapi" fn(Physical, u64) -> Status;

/// # References
/// * [EFI_REMOVE_MEMORY_SPACE](https://uefi.org/specs/PI/1.8/V2_Services_DXE_Services.html#removememoryspace)
pub type RemoveMemorySpace = extern "efiapi" fn(Physical, u64) -> Status;

/// # References
/// * [EFI_GET_MEMORY_SPACE_DESCRIPTOR](https://uefi.org/specs/PI/1.8/V2_Services_DXE_Services.html#getmemoryspacedescriptor)
pub type GetMemorySpaceDescriptor =
    extern "efiapi" fn(Physical, GcdMemorySpaceDescriptor) -> Status;

/// # References
/// * [EFI_GCD_MEMORY_SPACE_DESCRIPTOR](https://uefi.org/specs/PI/1.8/V2_Services_DXE_Services.html#getmemoryspacedescriptor)
#[repr(C)]
pub struct GcdMemorySpaceDescriptor {
    base_address: Physical,
    length: u64,
    capabilities: u64,
    attributes: u64,
    gcd_memory_type: GcdMemoryType,
    image: Handle,
    device: Handle,
}
