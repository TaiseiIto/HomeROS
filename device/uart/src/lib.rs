#![no_std]

mod ns16550a;
mod pl011;

use {
    arch::pause,
    core::{
        cell::OnceCell,
        fmt::{Arguments, Result, Write},
    },
    sync::spin::Lock,
};

#[cfg(has_device_tree)]
use tree::{Uart, uart::Standard};

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

pub enum Parity {
    Even,
    High,
    Low,
    Odd,
}

pub fn initialize(#[cfg(has_device_tree)] uart: Uart) {
    Abstract::new(
        #[cfg(has_device_tree)]
        uart,
    )
    .set();
}

pub static GLOBAL: Lock<OnceCell<Abstract>> = Lock::new(OnceCell::new());

#[derive(Debug)]
pub enum Abstract {
    Pl011(pl011::RegistersAccessor),
    Ns16550a(ns16550a::RegistersAccessor),
}

impl Abstract {
    pub fn write_format(&mut self, arguments: Arguments) {
        self.write_fmt(arguments).unwrap();
    }

    /// # TODO
    /// * Get address from device tree
    fn new(#[cfg(has_device_tree)] uart: Uart) -> Self {
        #[cfg(has_device_tree)]
        let mut accessor: Self = match uart.standard() {
            Standard::Pl011 => {
                Self::Pl011(unsafe { pl011::RegistersAccessor::new_address(uart.base_address()) })
            }
            Standard::Ns16550a => Self::Ns16550a(unsafe {
                ns16550a::RegistersAccessor::new_address(uart.base_address())
            }),
        };
        #[cfg(target_arch = "x86_64")]
        let mut accessor: Self =
            Self::Ns16550a(unsafe { ns16550a::RegistersAccessor::new_port(0x02f8) });
        let baud_rate: usize = 9600;
        let enable_fifo: bool = true;
        #[cfg(has_device_tree)]
        let frequency_hz: usize = uart.frequency_hz() as usize;
        #[cfg(target_arch = "x86_64")]
        let frequency_hz: usize = 115200;
        let parity: Option<Parity> = None;
        let send_break: bool = false;
        let stop_bits: u8 = 1;
        let word_bits: u8 = 8;
        accessor.initialize(
            baud_rate,
            enable_fifo,
            frequency_hz,
            parity,
            send_break,
            stop_bits,
            word_bits,
        );
        accessor
    }

    fn registers(&self) -> &dyn Driver {
        match self {
            Self::Ns16550a(driver) => driver,
            Self::Pl011(driver) => driver,
        }
    }

    fn registers_mut(&mut self) -> &mut dyn Driver {
        match self {
            Self::Ns16550a(driver) => driver,
            Self::Pl011(driver) => driver,
        }
    }

    fn set(self) {
        GLOBAL.lock().set(self).unwrap();
    }
}

impl Driver for Abstract {
    fn can_send_byte(&self) -> bool {
        self.registers().can_send_byte()
    }

    fn initialize(
        &mut self,
        baud_rate: usize,
        enable_fifo: bool,
        frequency_hz: usize,
        parity: Option<Parity>,
        send_break: bool,
        stop_bits: u8,
        word_bits: u8,
    ) {
        self.registers_mut().initialize(
            baud_rate,
            enable_fifo,
            frequency_hz,
            parity,
            send_break,
            stop_bits,
            word_bits,
        );
    }

    unsafe fn send_byte_unchecked(&mut self, data: u8) {
        self.registers_mut().send_byte(data);
    }
}

unsafe impl Sync for Abstract {}

impl Write for Abstract {
    fn write_str(&mut self, string: &str) -> Result {
        self.write_string(string);
        Ok(())
    }
}

trait Driver {
    fn can_send_byte(&self) -> bool;

    fn initialize(
        &mut self,
        baud_rate: usize,
        enable_fifo: bool,
        frequency_hz: usize,
        parity: Option<Parity>,
        send_break: bool,
        stop_bits: u8,
        word_bits: u8,
    );

    unsafe fn send_byte_unchecked(&mut self, data: u8);

    fn send_byte(&mut self, data: u8) {
        while !self.can_send_byte() {
            pause();
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
