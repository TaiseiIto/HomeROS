#![no_std]

use core::cell::OnceCell;

#[cfg(firmware = "uefi")]
pub use uefi;

static GLOBAL: SyncOnceCell<Global> = SyncOnceCell(OnceCell::new());

pub struct SyncOnceCell<T>(OnceCell<T>);

unsafe impl<T> Sync for SyncOnceCell<T> {}

/// # Safety
/// This function dereferences `image_handle` and `system_table`.
pub unsafe fn initialize_global(
    #[cfg(firmware = "uefi")] image_handle: uefi::HandleMut,
    #[cfg(firmware = "uefi")] system_table: *mut uefi::system::Table,
) {
    unsafe {
        GLOBAL
            .0
            .set(Global::new(
                #[cfg(firmware = "uefi")]
                image_handle,
                #[cfg(firmware = "uefi")]
                system_table,
            ))
            .unwrap();
    }
}

#[derive(Debug)]
struct Global {
    #[cfg(firmware = "uefi")]
    image_handle: uefi::HandleMut,
    #[cfg(firmware = "uefi")]
    system_table: &'static mut uefi::system::Table,
}

impl Global {
    /// # Safety
    /// This function dereferences `image_handle` and `system_table`.
    unsafe fn new(
        #[cfg(firmware = "uefi")] image_handle: uefi::HandleMut,
        #[cfg(firmware = "uefi")] system_table: *mut uefi::system::Table,
    ) -> Self {
        Self {
            #[cfg(firmware = "uefi")]
            image_handle,
            #[cfg(firmware = "uefi")]
            system_table: unsafe { &mut *system_table },
        }
    }
}
