#![no_std]

use core::{
    cell::OnceCell,
    fmt::{Arguments, Result, Write},
};

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::global().write_format(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => ($crate::print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::print!(concat!($fmt, "\n"), $($arg)*));
}

#[cfg(firmware = "uefi")]
pub use uefi;

static GLOBAL: SyncOnceCell<Global> = SyncOnceCell(OnceCell::new());

pub fn global() -> &'static Global {
    GLOBAL.0.get().unwrap()
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

    /// # Safety
    /// This function dereferences `image_handle` and `system_table`.
    pub unsafe fn set(self) {
        GLOBAL.0.set(self).unwrap();
    }

    pub fn write_format(&self, arguments: Arguments) {
        self.writer().write_fmt(arguments).unwrap();
    }

    fn writer(&self) -> Writer<'_> {
        Writer(self)
    }

    fn write_string(&self, string: &str) {
        #[cfg(firmware = "tfa")]
        rs232c::write_string(string);
        #[cfg(firmware = "sbi")]
        rs232c::write_string(string);
        #[cfg(firmware = "uefi")]
        self.system_table.write(string);
    }
}

pub struct Writer<'a>(&'a Global);

impl Write for Writer<'_> {
    fn write_str(&mut self, string: &str) -> Result {
        self.0.write_string(string);
        Ok(())
    }
}
