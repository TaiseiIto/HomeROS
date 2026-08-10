mod configuration;
mod controller;
mod event;
mod image;
pub mod memory;
mod protocol;
mod task;
mod timer;

use crate::{Char16, Status, Void, table};

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
    install_protocol: protocol::Install,
    reinstall_protocol: protocol::Reinstall,
    uninstall_protocol: protocol::Uninstall,
    handle_protocol: protocol::Handle,
    reserved: *const Void,
    register_protocol_notify: protocol::RegisterNotify,
    locate_handle: protocol::LocateHandle,
    locate_device_path: protocol::LocateDevicePath,
    install_configuration: configuration::Install,
    load_image: image::Load,
    start_image: image::Start,
    exit: image::Exit,
    unload_image: image::Unload,
    exit_services: image::ExitServices,
    get_next_monotonic_count: GetNextMonotonicCount,
    stall: Stall,
    set_watchdog_timer: SetWatchdogTimer,
    connect_controller: controller::Connect,
    disconnect_controller: controller::Disconnect,
}

/// References
/// * [GetNextMonotonicCount](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-getnextmonotoniccount)
pub type GetNextMonotonicCount = extern "efiapi" fn(*mut u64) -> Status;

/// References
/// * [SetWatchdogTimer](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-setwatchdogtimer)
pub type SetWatchdogTimer = extern "efiapi" fn(usize, u64, usize, *const Char16) -> Status;

/// References
/// * [Stall](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-stall)
pub type Stall = extern "efiapi" fn(usize) -> Status;
