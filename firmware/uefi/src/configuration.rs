mod lzma;

use {
    crate::{Guid, Void, service::dxe},
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
    const DXE: &str = "05ad34ba-6f02-4214-952e-4da0398e2bb9";
    const LZMA: &str = "ee4e5898-3914-4259-9d6e-dc7bd79403cf";
    /// # References
    /// * [Finding the RSDP on UEFI Enabled Systems](https://uefi.org/htmlspecs/ACPI_Spec_6_4_html/05_ACPI_Software_Programming_Model/ACPI_Software_Programming_Model.html#finding-the-rsdp-on-uefi-enabled-systems)
    const RSDP: &str = "8868e871-e4f1-11d3-bc22-0080c73c8881";

    pub fn table(&self) -> Result<&dyn Table, &Guid> {
        if self.guid == Self::RSDP.parse().unwrap() {
            Ok(unsafe { &*(self.table as *const acpi::root_system_description::Pointer) })
        } else if self.guid == Self::LZMA.parse().unwrap() {
            Ok(unsafe { &*(self.table as *const lzma::Table) })
        } else if self.guid == Self::DXE.parse().unwrap() {
            Ok(unsafe { &*(self.table as *const dxe::Table) })
        } else {
            Err(&self.guid)
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
impl Table for lzma::Table {}
