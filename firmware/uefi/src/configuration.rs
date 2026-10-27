mod lzma;
mod memory_type;

use {
    crate::{
        Guid, Void,
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
    /// * [DXE Services Table GUID](https://uefi.org/specs/PI/1.10/V2_DXE_Appendix_B.html#dxe-services-table-guid)
    const DXE: Guid = guid!("05ad34ba-6f02-4214-952e-4da0398e2bb9");

    /// # References
    /// * [HOB List GUID](https://uefi.org/specs/PI/1.10/V2_DXE_Appendix_B.html#hob-list-guid)
    const HOB: Guid = guid!("7739f24c-93d7-11d4-9a3a-0090273fc14d");

    const LZMA: Guid = guid!("ee4e5898-3914-4259-9d6e-dc7bd79403cf");

    const MEMORY_TYPE_INFORMATION: Guid = guid!("4c19049f-4137-4dd3-9c10-8b97a83ffdfa");

    /// # References
    /// * [Finding the RSDP on UEFI Enabled Systems](https://uefi.org/htmlspecs/ACPI_Spec_6_4_html/05_ACPI_Software_Programming_Model/ACPI_Software_Programming_Model.html#finding-the-rsdp-on-uefi-enabled-systems)
    const RSDP: Guid = guid!("8868e871-e4f1-11d3-bc22-0080c73c8881");

    pub fn table(&self) -> Result<&dyn Table, &Guid> {
        match self.guid {
            Self::DXE => Ok(unsafe { &*(self.table as *const dxe::Table) }),
            Self::HOB => Ok(unsafe { &*(self.table as *const hob::List) }),
            Self::LZMA => Ok(unsafe { &*(self.table as *const lzma::Table) }),
            Self::MEMORY_TYPE_INFORMATION => {
                Ok(unsafe { &*(self.table as *const memory_type::Table) })
            }
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

impl Table for acpi::root_system_description::Pointer {}
impl Table for dxe::Table {}
impl Table for hob::List {}
impl Table for lzma::Table {}
impl Table for memory_type::Table {}
