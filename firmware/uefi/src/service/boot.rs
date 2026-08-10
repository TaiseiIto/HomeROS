pub mod memory;
mod task;

use crate::table;

/// References
/// * [EFI_BOOT_SERVICES](https://uefi.org/specs/UEFI/2.11/04_EFI_System_Table.html#efi-boot-services)
#[repr(C)]
pub struct Table {
    hdr: table::Header,
    raise_tpl: task::Raise,
    restore_tpl: task::Restore,
    allocate_pages: memory::page::Allocate,
}
