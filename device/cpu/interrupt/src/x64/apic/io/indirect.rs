pub mod identification;
pub mod redirection;
pub mod version;

use {super::GLOBAL, alloc::vec::Vec, core::cell::OnceCell, sync::spin::Guard};

#[derive(Debug)]
pub struct Registers {
    identification: identification::Register,
    version: version::Register,
    redirection_entries: Vec<redirection::Entry>,
}

impl Registers {
    pub fn get() -> Self {
        let mut lock: Guard<OnceCell<&'static mut super::Registers>> = GLOBAL.0.lock();
        let registers: &mut super::Registers = lock.get_mut().unwrap();
        Self {
            identification: registers.identification(),
            version: registers.version(),
            redirection_entries: registers.redirection_entries(),
        }
    }
}
