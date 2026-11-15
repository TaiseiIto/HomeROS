mod configuration;
mod controller;
mod event;
pub mod image;
pub mod memory;
mod protocol;
mod task;
mod timer;

use {
    crate::{Char16, Handle, Status, Void, table},
    alloc::vec::Vec,
    core::ptr::null_mut,
};

/// # References
/// * [EFI_BOOT_SERVICES](https://uefi.org/specs/UEFI/2.11/04_EFI_System_Table.html#efi-boot-services)
#[derive(Debug)]
#[repr(C)]
pub struct Table {
    hdr: table::Header,
    raise_tpl: Option<task::Raise>,
    restore_tpl: Option<task::Restore>,
    allocate_pages: Option<memory::page::Allocate>,
    free_pages: Option<memory::page::Free>,
    get_memory_map: Option<memory::map::Get>,
    allocate_pool: Option<memory::pool::Allocate>,
    free_pool: Option<memory::pool::Free>,
    create_event: Option<event::Create>,
    set_timer: Option<timer::Set>,
    wait_for_event: Option<event::Wait>,
    signal_event: Option<event::Signal>,
    close_event: Option<event::Close>,
    check_event: Option<event::Check>,
    install_protocol: Option<protocol::Install>,
    reinstall_protocol: Option<protocol::Reinstall>,
    uninstall_protocol: Option<protocol::Uninstall>,
    handle_protocol: Option<protocol::Handle>,
    __: *const Void,
    register_protocol_notify: Option<protocol::RegisterNotify>,
    locate_handle: Option<protocol::LocateHandle>,
    locate_device_path: Option<protocol::LocateDevicePath>,
    install_configuration: Option<configuration::Install>,
    load_image: Option<image::Load>,
    start_image: Option<image::Start>,
    exit: Option<image::Exit>,
    unload_image: Option<image::Unload>,
    exit_boot_services: Option<image::ExitBootServices>,
    get_next_monotonic_count: Option<GetNextMonotonicCount>,
    stall: Option<Stall>,
    set_watchdog_timer: Option<SetWatchdogTimer>,
    connect_controller: Option<controller::Connect>,
    disconnect_controller: Option<controller::Disconnect>,
    open_protocol: Option<protocol::Open>,
    close_protocol: Option<protocol::Close>,
    open_protocol_information: Option<protocol::OpenInformation>,
    protocols_per_handle: Option<protocol::PerHandle>,
    locate_handle_buffer: Option<protocol::LocateHandleBuffer>,
    locate_protocol: Option<protocol::Locate>,
    install_multiple_protocol: Option<protocol::multiple::Install>,
    uninstall_multiple_protocol: Option<protocol::multiple::Uninstall>,
    calculate_crc32: Option<CalculateCrc32>,
    copy_mem: Option<memory::Copy>,
    set_mem: Option<memory::Set>,
    create_event_ex: Option<event::CreateEx>,
}

impl Table {
    pub fn exit_boot_services(&mut self, image: Handle) -> memory::Map {
        let memory_map: memory::Map = self.get_memory_map();
        (self.exit_boot_services.unwrap())(image, memory_map.key()).assert();
        memory_map
    }

    fn allocate_pool(&self, size: usize) -> Vec<u8> {
        let mut buffer: *mut Void = null_mut();
        (self.allocate_pool.unwrap())(
            memory::Type::LoaderData,
            size,
            (&mut buffer) as *mut *mut Void,
        )
        .assert();
        unsafe { Vec::<u8>::from_raw_parts(buffer as *mut u8, size, size) }
    }

    fn get_memory_map(&self) -> memory::Map {
        let mut size: usize = 2 * self.get_memory_map_size();
        let mut descriptors: Vec<u8> = self.allocate_pool(size);
        let mut key: usize = 0;
        let mut descriptor_size: usize = 0;
        let mut descriptor_version: u32 = 0;
        (self.get_memory_map.unwrap())(
            (&mut size) as *mut usize,
            descriptors.as_mut_ptr() as *mut memory::map::Descriptor,
            (&mut key) as *mut usize,
            (&mut descriptor_size) as *mut usize,
            (&mut descriptor_version) as *mut u32,
        )
        .assert();
        descriptors.truncate(size);
        memory::Map::new(key, descriptors, descriptor_size)
    }

    fn get_memory_map_size(&self) -> usize {
        let mut size: usize = 0;
        let mut key: usize = 0;
        let mut descriptor_size: usize = 0;
        let mut descriptor_version: u32 = 0;
        (self.get_memory_map.unwrap())(
            (&mut size) as *mut usize,
            null_mut(),
            (&mut key) as *mut usize,
            (&mut descriptor_size) as *mut usize,
            (&mut descriptor_version) as *mut u32,
        )
        .assert_buffer_too_small();
        size
    }
}

/// # References
/// * [CalculateCrc32](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-calculatecrc32)
pub type CalculateCrc32 = extern "efiapi" fn(*const Void, usize, *mut u32) -> Status;

/// # References
/// * [GetNextMonotonicCount](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-getnextmonotoniccount)
pub type GetNextMonotonicCount = extern "efiapi" fn(*mut u64) -> Status;

/// # References
/// * [SetWatchdogTimer](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-setwatchdogtimer)
pub type SetWatchdogTimer = extern "efiapi" fn(usize, u64, usize, *const Char16) -> Status;

/// # References
/// * [Stall](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-stall)
pub type Stall = extern "efiapi" fn(usize) -> Status;
