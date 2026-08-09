use crate::table;

mod time;

/// # References
/// * [EFI_RUNTIME_SERVICES](https://uefi.org/specs/UEFI/2.11/04_EFI_System_Table.html#efi-runtime-services)
#[repr(C)]
pub struct Runtime {
    hdr: table::Header,
    get_time: time::Get,
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
