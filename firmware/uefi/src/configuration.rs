mod debug_image;
mod lzma;
mod memory;

use {
    crate::{
        Guid, Void, acpi, guid,
        pi::{dxe, hob},
    },
    core::fmt::{self, Debug, Formatter},
};

/// # References
/// * [EFI_CONFIGURATION_TABLE](https://uefi.org/specs/UEFI/2.11/04_EFI_System_Table.html#efi-configuration-table)
/// # TODO
/// * Search all GUID appearing on the configuration tables.
#[repr(C)]
pub struct Pointer {
    guid: Guid,
    table: *const Void,
}

impl Pointer {
    /// # References
    /// * [SMBIOS Specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.10.0.pdf) 5.2.1 SMBIOS 2.1 (32-bit) Entry Point
    const SMBIOS32: Guid = guid!("eb9d2d31-2d88-11d3-9a16-0090273fc14d");

    /// # References
    /// * [SMBIOS Specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.10.0.pdf) 5.2.2 SMBIOS 3.0 (64-bit) Entry Point
    const SMBIOS64: Guid = guid!("f2fd1544-9794-4a2c-992e-e5bbcf20e394");

    pub fn table(&self) -> Result<&dyn Table, &Guid> {
        match self.guid {
            acpi::root_system_description::Pointer::VERSION1
            | acpi::root_system_description::Pointer::VERSION2 => {
                Ok(unsafe { &*(self.table as *const acpi::root_system_description::Pointer) })
            }
            debug_image::Table::GUID => Ok(unsafe { &*(self.table as *const debug_image::Table) }),
            dxe::Table::GUID => Ok(unsafe { &*(self.table as *const dxe::Table) }),
            hob::List::GUID => Ok(unsafe { &*(self.table as *const hob::List) }),
            lzma::Table::GUID => Ok(unsafe { &*(self.table as *const lzma::Table) }),
            memory::status_code::Record::GUID => {
                Ok(unsafe { &*(self.table as *const memory::status_code::Record) })
            }
            memory::typ::Table::GUID => Ok(unsafe { &*(self.table as *const memory::typ::Table) }),
            Self::SMBIOS32 => Ok(unsafe { &*(self.table as *const smbios::EntryPoint32) }),
            Self::SMBIOS64 => Ok(unsafe { &*(self.table as *const smbios::EntryPoint64) }),
            _ => Err(&self.guid),
        }
    }
}

impl Debug for Pointer {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_fmt(format_args!("{:#x?}", self.table()))
    }
}

pub trait Table: Debug {}

impl Table for smbios::EntryPoint32 {}
impl Table for smbios::EntryPoint64 {}
