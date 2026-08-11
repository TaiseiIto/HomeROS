use crate::Status;

/// # References
/// * [EFI_TIME_CAPABILITIES](https://uefi.org/specs/UEFI/2.11/08_Services_Runtime_Services.html#gettime)
#[repr(C)]
pub struct Capabilities {
    resolution: u32,
    accuracy: u32,
    set_to_zero: bool,
}

/// # References
/// * [EFI_TIME](https://uefi.org/specs/UEFI/2.11/08_Services_Runtime_Services.html#gettime)
#[repr(C)]
pub struct Time {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    pad1: u8,
    nanosecond: u32,
    timezone: i16,
    dailight: u8,
    pad2: u8,
}

/// # References
/// * [GetTime](https://uefi.org/specs/UEFI/2.11/08_Services_Runtime_Services.html#gettime)
pub type Get = extern "efiapi" fn(*mut Time, *mut Capabilities) -> Status;

/// # References
/// * [GetWakeupTime](https://uefi.org/specs/UEFI/2.11/08_Services_Runtime_Services.html#getwakeuptime)
pub type GetWakeup = extern "efiapi" fn(*mut bool, *mut bool, *mut Time) -> Status;

/// # References
/// * [SetTime](https://uefi.org/specs/UEFI/2.11/08_Services_Runtime_Services.html#settime)
pub type Set = extern "efiapi" fn(*const Time) -> Status;

/// # References
/// * [SetWakeupTime](https://uefi.org/specs/UEFI/2.11/08_Services_Runtime_Services.html#setwakeuptime)
pub type SetWakeup = extern "efiapi" fn(bool, *const Time) -> Status;
