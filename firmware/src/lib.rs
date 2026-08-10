#![no_std]

#[cfg(firmware = "uefi")]
pub use uefi;

pub struct Global {
    #[cfg(firmware = "uefi")]
    image_handle: uefi::Handle,
    #[cfg(firmware = "uefi")]
    system_table: &'static uefi::system::Table,
}

impl Global {
    pub unsafe fn new(
        #[cfg(firmware = "uefi")] image_handle: uefi::Handle,
        #[cfg(firmware = "uefi")] system_table: *const uefi::system::Table,
    ) -> Self {
        Self {
            #[cfg(firmware = "uefi")]
            image_handle,
            #[cfg(firmware = "uefi")]
            system_table: unsafe { &*system_table },
        }
    }
}
