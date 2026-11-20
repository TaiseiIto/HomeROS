mod data;
mod eoi;
mod index;
pub mod indirect;

use {
    alloc::vec::Vec,
    core::{cell::OnceCell, mem::size_of},
    sync::spin::Lock,
};

pub fn initialize(address: usize) {
    GLOBAL
        .0
        .lock()
        .set(unsafe { &mut *(address as *mut Registers) });
}

static GLOBAL: Global = Global::new();

struct Global(Lock<OnceCell<&'static mut Registers>>);

impl Global {
    const fn new() -> Self {
        Self(Lock::new(OnceCell::new()))
    }
}

/// # References
/// * [Intel 600 Series and Intel 700 Series Chipset Family Platform Controller Hub (PCH) Datasheet - Volume 2 of 2](https://www.intel.com/content/www/us/en/content-details/680836/intel-600-series-and-intel-700-series-chipset-family-platform-controller-hub-pch-datasheet-volume-2-of-2.html) 24.2 Advanced Programmable Interrupt Controller (APIC) Registers Summary
#[io::registers()]
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
    fn identification(&mut self) -> indirect::identification::Register {
        unsafe {
            self.write_index_memory(index::Register::default().update_index_u8(0));
            (&*(&self.data as *const data::RegisterRaw
                as *const indirect::identification::RegisterRaw))
                .read_memory()
        }
    }

    fn redirection_entries(&mut self) -> Vec<indirect::redirection::Entry> {
        (0..self.redirection_entries_length())
            .map(|index| self.redirection_entry(index))
            .collect()
    }

    fn redirection_entries_length(&mut self) -> usize {
        self.version().read_maximum_redirection_entries_u8() as usize + 1
    }

    fn redirection_entry(&mut self, index: usize) -> indirect::redirection::Entry {
        let low: u8 = (2 * index + 0x10) as u8;
        let high: u8 = low + 1;
        indirect::redirection::Entry::new(
            unsafe {
                self.write_index_memory(index::Register::default().update_index_u8(low));
                (&*(&self.data as *const data::RegisterRaw as *const indirect::redirection::LowRaw))
                    .read_memory()
            },
            unsafe {
                self.write_index_memory(index::Register::default().update_index_u8(high));
                (&*(&self.data as *const data::RegisterRaw
                    as *const indirect::redirection::HighRaw))
                    .read_memory()
            },
        )
    }

    fn version(&mut self) -> indirect::version::Register {
        unsafe {
            self.write_index_memory(index::Register::default().update_index_u8(1));
            (&*(&self.data as *const data::RegisterRaw as *const indirect::version::RegisterRaw))
                .read_memory()
        }
    }
}
