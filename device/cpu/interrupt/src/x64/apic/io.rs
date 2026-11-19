mod data;
mod eoi;
mod identification;
mod index;

use {
    core::{cell::OnceCell, mem::size_of},
    sync::spin::Lock,
};

pub fn initialize(address: usize) {
    GLOBAL
        .0
        .lock()
        .set(unsafe { &mut *(address as *mut Registers) });
}

pub static GLOBAL: Global = Global::new();

pub struct Global(Lock<OnceCell<&'static mut Registers>>);

impl Global {
    const fn new() -> Self {
        Self(Lock::new(OnceCell::new()))
    }
}

/// # References
/// * [Intel 600 Series and Intel 700 Series Chipset Family Platform Controller Hub (PCH) Datasheet - Volume 2 of 2](https://www.intel.com/content/www/us/en/content-details/680836/intel-600-series-and-intel-700-series-chipset-family-platform-controller-hub-pch-datasheet-volume-2-of-2.html) 24.2 Advanced Programmable Interrupt Controller (APIC) Registers Summary
/// # TODO
/// * Print APIC Indirect Registers
#[io::registers]
pub struct Registers {
    index: index::Register,
    __: [u8; 0xc],
    data: data::Register,
    __: [u8; 0x2c],
    eoi: eoi::Register,
    __: [u8; 0xc],
}

const _: () = assert!(size_of::<Registers>() == 0x50);

impl Registers {
    pub fn identification(&mut self) -> identification::Register {
        unsafe {
            self.write_index_memory(index::Register::default().update_index_u8(0));
            (&*(&self.data as *const data::RegisterRaw as *const identification::RegisterRaw))
                .read_memory()
        }
    }
}
