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
