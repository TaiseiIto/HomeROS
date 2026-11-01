mod debug_image;
mod lzma;
mod memory;

use {
    crate::{
        Guid, Void, acpi,
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
    pub fn table(&self) -> Result<&dyn Table, &Guid> {
        match self.guid {
            debug_image::Table::GUID => Ok(unsafe { &*(self.table as *const debug_image::Table) }),
            dxe::Table::GUID => Ok(unsafe { &*(self.table as *const dxe::Table) }),
            hob::List::GUID => Ok(unsafe { &*(self.table as *const hob::List) }),
            lzma::Table::GUID => Ok(unsafe { &*(self.table as *const lzma::Table) }),
            memory::status_code::Record::GUID => {
                Ok(unsafe { &*(self.table as *const memory::status_code::Record) })
            }
            memory::typ::Table::GUID => Ok(unsafe { &*(self.table as *const memory::typ::Table) }),
            acpi::root_system_description::Pointer::GUID => {
                Ok(unsafe { &*(self.table as *const acpi::root_system_description::Pointer) })
            }
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
