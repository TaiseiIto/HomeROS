use crate::{Guid, Void};

/// # References
/// * [EFI_CONFIGURATION_TABLE](https://uefi.org/specs/UEFI/2.11/04_EFI_System_Table.html#efi-configuration-table)
/// # TODO
/// * Search all GUID appearing on the configuration tables.
#[derive(Debug)]
#[repr(C)]
pub struct Pointer {
    guid: Guid,
    table: *const Void,
}

impl Pointer {
    /// # References
    /// * [Finding the RSDP on UEFI Enabled Systems](https://uefi.org/htmlspecs/ACPI_Spec_6_4_html/05_ACPI_Software_Programming_Model/ACPI_Software_Programming_Model.html#finding-the-rsdp-on-uefi-enabled-systems)
    const RSDP: &str = "8868e871-e4f1-11d3-bc22-0080c73c8881";
}

impl<'a> TryFrom<&'a Pointer> for &'a acpi::root_system_description::Pointer {
    type Error = ();

    fn try_from(table: &'a Pointer) -> Result<Self, Self::Error> {
        (table.guid == Pointer::RSDP.parse().unwrap())
            .then(|| unsafe { &*(table.table as *const acpi::root_system_description::Pointer) })
            .inspect(|pointer| assert!(pointer.is_correct()))
            .ok_or(())
    }
}
