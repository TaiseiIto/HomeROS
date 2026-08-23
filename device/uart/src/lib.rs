#![feature(sync_unsafe_cell)]
#![no_std]

#[cfg(uart = "pl011")]
mod pl011;
#[cfg(uart = "pl011")]
use pl011::RegistersAccessor;
#[cfg(uart = "16550")]
mod standard16550;
#[cfg(uart = "16550")]
use standard16550::RegistersAccessor;

use core::{
    arch::asm,
    cell::{OnceCell, SyncUnsafeCell},
    fmt::{Arguments, Result, Write},
};

#[macro_export]
macro_rules! dbg {
    ($arg:expr) => {{
        let value = $arg;
        $crate::println!(
            "[{}:{}:{}] {} = {}",
            file!(),
            line!(),
            column!(),
            stringify!($arg),
            value
        );
        value
    }};
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::global_mut().write_format(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => ($crate::print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::print!(concat!($fmt, "\n"), $($arg)*));
}

pub fn initialize() {
    RegistersAccessor::new().set();
}

static GLOBAL: SyncUnsafeCell<SyncOnceCell<RegistersAccessor>> =
    SyncUnsafeCell::new(SyncOnceCell(OnceCell::new()));

pub fn global_mut() -> &'static mut RegistersAccessor {
    unsafe { &mut *GLOBAL.get() }.0.get_mut().unwrap()
}

impl RegistersAccessor {
    pub fn new() -> Self {
        #[cfg(target_arch = "aarch64")]
        let mut accessor: Self = unsafe { Self::new_address(0x09000000) };
        #[cfg(target_arch = "riscv64")]
        let mut accessor: Self = unsafe { Self::new_address(0x10000000) };
        #[cfg(target_arch = "x86_64")]
        let mut accessor: Self = unsafe { Self::new_port(0x03f8) };
        let baud_rate: usize = 9600;
        let enable_fifo: bool = true;
        let parity: Option<Parity> = None;
        let send_break: bool = false;
        let stop_bits: u8 = 1;
        let word_bits: u8 = 8;
        accessor.initialize(
            baud_rate,
            enable_fifo,
            parity,
            send_break,
            stop_bits,
            word_bits,
        );
        accessor
    }

    pub fn set(self) {
        unsafe { &mut *GLOBAL.get() }.0.set(self).unwrap();
    }

    pub fn write_format(&mut self, arguments: Arguments) {
        self.write_fmt(arguments).unwrap();
    }
}

unsafe impl Sync for RegistersAccessor {}

impl Write for RegistersAccessor {
    fn write_str(&mut self, string: &str) -> Result {
        self.write_string(string);
        Ok(())
    }
}

enum Parity {
    Even,
    High,
    Low,
    Odd,
}

/// # TODO
/// This is not thread safe actualty.
/// Make it thread safe.
struct SyncOnceCell<T>(OnceCell<T>);

unsafe impl<T> Sync for SyncOnceCell<T> {}

pub trait Driver {
    fn can_send_byte(&self) -> bool;

    fn initialize(
        &mut self,
        baud_rate: usize,
        enable_fifo: bool,
        parity: Option<Parity>,
        send_break: bool,
        stop_bits: u8,
        word_bits: u8,
    );

    unsafe fn send_byte_unchecked(&mut self, data: u8);

    fn send_byte(&mut self, data: u8) {
        while !self.can_send_byte() {
            arch::pause();
        }
        unsafe {
            self.send_byte_unchecked(data);
        }
    }

    fn write_string(&mut self, string: &str) {
        for byte in string.bytes() {
            self.send_byte(byte);
        }
    }
}
