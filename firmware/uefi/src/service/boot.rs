mod event;
pub mod memory;
mod task;
mod timer;

use crate::table;

/// References
/// * [EFI_BOOT_SERVICES](https://uefi.org/specs/UEFI/2.11/04_EFI_System_Table.html#efi-boot-services)
#[repr(C)]
pub struct Table {
    hdr: table::Header,
    raise_tpl: task::Raise,
    restore_tpl: task::Restore,
    allocate_pages: memory::page::Allocate,
    free_pages: memory::page::Free,
    get_memory_map: memory::map::Get,
    allocate_pool: memory::pool::Allocate,
    free_pool: memory::pool::Free,
    create_event: event::Create,
    set_timer: timer::Set,
    wait_for_event: event::Wait,
    signal_event: event::Signal,
    close_event: event::Close,
    check_event: event::Check,
}
