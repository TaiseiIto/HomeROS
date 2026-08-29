#![no_std]

use {
    core::{
        cell::OnceCell,
        fmt::{Arguments, Result, Write},
    },
    sync::spin::Lock,
};

#[macro_export]
macro_rules! dbg {
    ($arg:expr) => {
        match $arg {
            tmp => {
                $crate::println!(
                    "[{}:{}:{}] {} = {:#x?}",
                    file!(),
                    line!(),
                    column!(),
                    stringify!($arg),
                    tmp
                );
                tmp
            }
        }
    };
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::GLOBAL.lock().get_mut().unwrap().write_format(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => ($crate::print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::print!(concat!($fmt, "\n"), $($arg)*));
}

#[cfg(firmware = "uefi")]
pub use uefi;

pub static GLOBAL: Lock<OnceCell<Global>> = Lock::new(OnceCell::new());

#[derive(Debug)]
pub struct Global {
    #[cfg(firmware = "sbi")]
    hartid: usize,
    #[cfg(any(firmware = "sbi", firmware = "tfa"))]
    device_tree: &'static tree::Header,
    #[cfg(firmware = "uefi")]
    image_handle: uefi::HandleMut,
    #[cfg(firmware = "uefi")]
    system_table: &'static mut uefi::system::Table,
}

impl Global {
    /// # Safety
    /// This function dereferences raw pointers.
    /// Caller must pass valid pointers.
    pub unsafe fn new(
        #[cfg(firmware = "sbi")] hartid: usize,
        #[cfg(firmware = "sbi")] device_tree: *const tree::Header,
        #[cfg(firmware = "uefi")] image_handle: uefi::HandleMut,
        #[cfg(firmware = "uefi")] system_table: *mut uefi::system::Table,
    ) -> Self {
        Self {
            #[cfg(firmware = "sbi")]
            hartid,
            #[cfg(firmware = "sbi")]
            device_tree: unsafe { &*device_tree },
            #[cfg(firmware = "tfa")]
            device_tree: unsafe { &*(0x40000000 as *const tree::Header) },
            #[cfg(firmware = "uefi")]
            image_handle,
            #[cfg(firmware = "uefi")]
            system_table: unsafe { &mut *system_table },
        }
    }

    pub fn set(self) {
        GLOBAL.lock().set(self).unwrap();
    }

    pub fn write_format(&mut self, arguments: Arguments) {
        self.write_fmt(arguments).unwrap();
    }

    #[cfg(firmware = "sbi")]
    fn write_byte(&self, byte: u8) {
        sbi::console::putchar(byte);
    }

    fn write_string(&self, string: &str) {
        #[cfg(firmware = "sbi")]
        for byte in string.bytes() {
            self.write_byte(byte);
        }
        #[cfg(firmware = "tfa")]
        uart::GLOBAL
            .lock()
            .get_mut()
            .unwrap()
            .write_str(string)
            .unwrap();
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

unsafe impl Send for Global {}
unsafe impl Sync for Global {}
