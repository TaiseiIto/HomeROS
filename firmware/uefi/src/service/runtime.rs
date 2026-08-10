use {
    super::{memory, time, variable},
    crate::{Status, Void, table},
};

/// # References
/// * [EFI_RUNTIME_SERVICES](https://uefi.org/specs/UEFI/2.11/04_EFI_System_Table.html#efi-runtime-services)
#[repr(C)]
pub struct Runtime {
    hdr: table::Header,
    get_time: time::Get,
    set_time: time::Set,
    get_wakeup_time: time::GetWakeup,
    set_wakeup_time: time::SetWakeup,
    set_virtual_address_map: memory::address::Set,
    convert_pointer: memory::address::Get,
    get_variable: variable::Get,
    get_next_variable_name: variable::Next,
    set_variable: variable::Set,
    get_next_high_monotonic_count: GetNextHighMonotonicCount,
    reset_system: ResetSystem,
}

/// # References
/// * [GetNextHighMonotonicCount](https://uefi.org/specs/UEFI/2.11/08_Services_Runtime_Services.html#getnexthighmonotoniccount)
pub type GetNextHighMonotonicCount = extern "efiapi" fn(*mut u32) -> Status;

/// # References
/// * [ResetSystem](https://uefi.org/specs/UEFI/2.11/08_Services_Runtime_Services.html#resetsystem)
pub type ResetSystem = extern "efiapi" fn(ResetType, Status, usize, *const Void) -> Status;

/// # References
/// * [EFI_RESET_TYPE](https://uefi.org/specs/UEFI/2.11/08_Services_Runtime_Services.html#resetsystem)
#[repr(C)]
pub enum ResetType {
    Cold,
    Warm,
    Shutdown,
    PlatformSpecific,
}
