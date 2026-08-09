use crate::table;

mod time;

/// # References
/// * [EFI_RUNTIME_SERVICES](https://uefi.org/specs/UEFI/2.11/04_EFI_System_Table.html#efi-runtime-services)
#[repr(C)]
pub struct Runtime {
    hdr: table::Header,
    get_time: time::Get,
    set_time: time::Set,
    get_wakeup_time: time::GetWakeup,
    set_wakeup_time: time::SetWakeup,
}
