#![feature(sync_unsafe_cell)]
#![no_std]

use core::{
    cell::{OnceCell, SyncUnsafeCell},
    fmt::{Arguments, Result, Write},
};

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::global_mut().write_format(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => ($crate::print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::print!(concat!($fmt, "\n"), $($arg)*));
}

#[cfg(firmware = "uefi")]
pub use uefi;

static GLOBAL: SyncUnsafeCell<SyncOnceCell<Global>> =
    SyncUnsafeCell::new(SyncOnceCell(OnceCell::new()));

pub fn global_mut() -> &'static mut Global {
    unsafe { &mut *GLOBAL.get() }.0.get_mut().unwrap()
}

/// # TODO
/// This is not thread safe actualty.
/// Make it thread safe.
pub struct SyncOnceCell<T>(OnceCell<T>);

unsafe impl<T> Sync for SyncOnceCell<T> {}

#[derive(Debug)]
pub struct Global {
    #[cfg(firmware = "uefi")]
    image_handle: uefi::HandleMut,
    #[cfg(firmware = "uefi")]
    system_table: &'static mut uefi::system::Table,
}

impl Global {
    /// # Safety
    /// This function dereferences `image_handle` and `system_table`.
    pub unsafe fn new(
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

    pub fn set(self) {
        unsafe { &mut *GLOBAL.get() }.0.set(self).unwrap();
    }

    pub fn write_format(&mut self, arguments: Arguments) {
        self.write_fmt(arguments).unwrap();
    }

    #[cfg(any(firmware = "sbi", firmware = "tfa"))]
    fn write_byte(&self, byte: u8) {
        #[cfg(firmware = "sbi")]
        sbi::console::putchar(byte);
        #[cfg(firmware = "tfa")]
        uart::write_byte(byte);
    }

    fn write_string(&self, string: &str) {
        #[cfg(any(firmware = "sbi", firmware = "tfa"))]
        for byte in string.bytes() {
            self.write_byte(byte);
        }
        #[cfg(firmware = "uefi")]
        self.system_table.write(string);
    }
}

impl Write for Global {
    fn write_str(&mut self, string: &str) -> Result {
        self.write_string(string);
        Ok(())
    }
}
