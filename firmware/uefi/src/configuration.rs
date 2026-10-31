mod debug_image;
mod lzma;
mod memory_type;

use {
    crate::{
        Guid, Void, acpi,
        pi::{dxe, hob},
    },
    core::fmt::{self, Debug, Formatter},
    guid::guid,
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
    /// * [Finding the RSDP on UEFI Enabled Systems](https://uefi.org/htmlspecs/ACPI_Spec_6_4_html/05_ACPI_Software_Programming_Model/ACPI_Software_Programming_Model.html#finding-the-rsdp-on-uefi-enabled-systems)
    const RSDP: Guid = guid!("8868e871-e4f1-11d3-bc22-0080c73c8881");

    pub fn table(&self) -> Result<&dyn Table, &Guid> {
        match self.guid {
            debug_image::Table::GUID => Ok(unsafe { &*(self.table as *const debug_image::Table) }),
            dxe::Table::GUID => Ok(unsafe { &*(self.table as *const dxe::Table) }),
            hob::List::GUID => Ok(unsafe { &*(self.table as *const hob::List) }),
            lzma::Table::GUID => Ok(unsafe { &*(self.table as *const lzma::Table) }),
            memory_type::Table::GUID => Ok(unsafe { &*(self.table as *const memory_type::Table) }),
            Self::RSDP => {
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

impl Table for debug_image::Table {}
impl Table for dxe::Table {}
impl Table for hob::List {}
impl Table for lzma::Table {}
impl Table for memory_type::Table {}
