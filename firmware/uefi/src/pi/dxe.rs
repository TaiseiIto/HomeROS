use {
    crate::{
        Guid, Handle, Status, Void, configuration, service::boot::memory::address::Physical, table,
    },
    guid::guid,
};

/// # References
/// * [DXE_SERVICES](https://uefi.org/specs/PI/1.10/V2_UEFI_System_Table.html#dxe-services)
#[derive(Debug)]
#[repr(C)]
pub struct Table {
    hdr: table::Header,
    add_memory_space: Option<AddMemorySpace>,
    allocate_memory_space: Option<AllocateMemorySpace>,
    free_memory_space: Option<FreeMemorySpace>,
    remove_memory_space: Option<RemoveMemorySpace>,
    get_memory_space_descriptor: Option<GetMemorySpaceDescriptor>,
    set_memory_space_attributes: Option<SetMemorySpaceAttributes>,
    get_memory_space_map: Option<GetMemorySpaceMap>,
    add_io_space: Option<AddIoSpace>,
    allocate_io_space: Option<AllocateIoSpace>,
    free_io_space: Option<FreeIoSpace>,
    remove_io_space: Option<RemoveIoSpace>,
    get_io_space_descriptor: Option<GetIoSpaceDescriptor>,
    get_io_space_map: Option<GetIoSpaceMap>,
    dispatch: Option<Dispatch>,
    schedule: Option<Schedule>,
    truct: Option<Trust>,
    process_firmware_volume: Option<ProcessFirmwareVolume>,
    set_memory_space_capabilities: Option<SetMemorySpaceCapabilities>,
}

impl Table {
    /// # References
    /// * [DXE_SERVICES_TABLE_GUID](https://uefi.org/specs/PI/1.10/V2_DXE_Appendix_B.html#dxe-services-table-guid)
    pub const GUID: Guid = guid!("05ad34ba-6f02-4214-952e-4da0398e2bb9");
}

impl configuration::Table for Table {}

/// # References
/// * [EFI_ADD_MEMORY_SPACE](https://uefi.org/specs/PI/1.10/V2_Services_DXE_Services.html#addmemoryspace)
pub type AddMemorySpace = extern "efiapi" fn(GcdMemoryType, Physical, u64, u64) -> Status;

/// # References
/// * [EFI_GCD_MEMORY_TYPE](https://uefi.org/specs/PI/1.10/V2_Services_DXE_Services.html#addmemoryspace)
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
/// * [EFI_ALLOCATE_MEMORY_SPACE](https://uefi.org/specs/PI/1.10/V2_Services_DXE_Services.html#allocatememoryspace)
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
/// * [EFI_GCD_ALLOCATE_TYPE](https://uefi.org/specs/PI/1.10/V2_Services_DXE_Services.html#allocatememoryspace)
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
/// * [EFI_FREE_MEMORY_SPACE](https://uefi.org/specs/PI/1.10/V2_Services_DXE_Services.html#freememoryspace)
pub type FreeMemorySpace = extern "efiapi" fn(Physical, u64) -> Status;

/// # References
/// * [EFI_REMOVE_MEMORY_SPACE](https://uefi.org/specs/PI/1.10/V2_Services_DXE_Services.html#removememoryspace)
pub type RemoveMemorySpace = extern "efiapi" fn(Physical, u64) -> Status;

/// # References
/// * [EFI_GET_MEMORY_SPACE_DESCRIPTOR](https://uefi.org/specs/PI/1.10/V2_Services_DXE_Services.html#getmemoryspacedescriptor)
pub type GetMemorySpaceDescriptor =
    extern "efiapi" fn(Physical, GcdMemorySpaceDescriptor) -> Status;

/// # References
/// * [EFI_GCD_MEMORY_SPACE_DESCRIPTOR](https://uefi.org/specs/PI/1.10/V2_Services_DXE_Services.html#getmemoryspacedescriptor)
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

/// # References
/// * [EFI_SET_MEMORY_SPACE_ATTRIBUTES](https://uefi.org/specs/PI/1.10/V2_Services_DXE_Services.html#setmemoryspaceattributes)
pub type SetMemorySpaceAttributes = extern "efiapi" fn(Physical, u64, u64) -> Status;

/// # References
/// * [EFI_SET_MEMORY_SPACE_CAPABILITIES](https://uefi.org/specs/PI/1.10/V2_Services_DXE_Services.html#setmemoryspacecapabilities)
pub type SetMemorySpaceCapabilities = extern "efiapi" fn(Physical, u64, u64) -> Status;

/// # References
/// * [EFI_GET_MEMORY_SPACE_MAP](https://uefi.org/specs/PI/1.10/V2_Services_DXE_Services.html#getmemoryspacemap)
pub type GetMemorySpaceMap =
    extern "efiapi" fn(*mut usize, *mut *mut GcdMemorySpaceDescriptor) -> Status;

/// # References
/// * [EFI_ADD_IO_SPACE](https://uefi.org/specs/PI/1.10/V2_Services_DXE_Services.html#addiospace)
pub type AddIoSpace = extern "efiapi" fn(GcdIoType, Physical, u64) -> Status;

/// # References
/// * [EFI_GCD_IO_TYPE](https://uefi.org/specs/PI/1.10/V2_Services_DXE_Services.html#addiospace)
#[derive(Debug)]
#[repr(C)]
pub enum GcdIoType {
    NonExistent,
    Reserved,
    Io,
    Maximum,
}

/// # References
/// * [EFI_ALLOCATE_IO_SPACE](https://uefi.org/specs/PI/1.10/V2_Services_DXE_Services.html#allocateiospace)
pub type AllocateIoSpace = extern "efiapi" fn(
    GcdAllocateType,
    GcdIoType,
    usize,
    u64,
    *mut Physical,
    image: Handle,
    device: Handle,
) -> Status;

/// # References
/// * [EFI_FREE_IO_SPACE](https://uefi.org/specs/PI/1.10/V2_Services_DXE_Services.html#freeiospace)
pub type FreeIoSpace = extern "efiapi" fn(Physical, u64) -> Status;

/// # References
/// * [EFI_REMOVE_IO_SPACE](https://uefi.org/specs/PI/1.10/V2_Services_DXE_Services.html#removeiospace)
pub type RemoveIoSpace = extern "efiapi" fn(Physical, u64) -> Status;

/// # References
/// * [EFI_GET_IO_SPACE_DESCRIPTOR](https://uefi.org/specs/PI/1.10/V2_Services_DXE_Services.html#getiospacedescriptor)
pub type GetIoSpaceDescriptor = extern "efiapi" fn(Physical, *mut GcdIoSpaceDescriptor) -> Status;

/// # References
/// * [EFI_GCD_IO_SPACE_DESCRIPTOR](https://uefi.org/specs/PI/1.10/V2_Services_DXE_Services.html#getiospacedescriptor)
#[repr(C)]
pub struct GcdIoSpaceDescriptor {
    base_address: Physical,
    length: u64,
    gcd_io_type: GcdIoType,
    image: Handle,
    device: Handle,
}

/// # References
/// * [EFI_GET_IO_SPACE_MAP](https://uefi.org/specs/PI/1.10/V2_Services_DXE_Services.html#getiospacemap)
pub type GetIoSpaceMap = extern "efiapi" fn(*mut usize, *mut *mut GcdIoSpaceDescriptor) -> Status;

/// # References
/// * [EFI_DISPATCH](https://uefi.org/specs/PI/1.10/V2_Services_DXE_Services.html#dispatch)
pub type Dispatch = extern "efiapi" fn() -> Status;

/// # References
/// * [EFI_SCHEDULE](https://uefi.org/specs/PI/1.10/V2_Services_DXE_Services.html#schedule)
pub type Schedule = extern "efiapi" fn(Handle, *const Guid) -> Status;

/// # References
/// * [EFI_TRUST](https://uefi.org/specs/PI/1.10/V2_Services_DXE_Services.html#trust)
pub type Trust = extern "efiapi" fn(Handle, *const Guid) -> Status;

/// # References
/// * [EFI_PROCESS_FIRMWARE_VOLUME](https://uefi.org/specs/PI/1.10/V2_Services_DXE_Services.html#trust)
pub type ProcessFirmwareVolume = extern "efiapi" fn(*const Void, usize, *mut Handle) -> Status;
