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
    const RSDP: Guid = Guid {
        data1: 0x8868e871,
        data2: 0xe4f1,
        data3: 0x11d3,
        data4: [0xbc, 0x22, 0x00, 0x80, 0xc7, 0x3c, 0x88, 0x81],
    };
}

impl<'a> TryFrom<&'a Pointer> for &'a acpi::root_system_description::Pointer {
    type Error = ();

    fn try_from(table: &'a Pointer) -> Result<Self, Self::Error> {
        (table.guid == Pointer::RSDP)
            .then(|| unsafe { &*(table.table as *const acpi::root_system_description::Pointer) })
            .inspect(|pointer| assert!(pointer.is_correct()))
            .ok_or(())
    }
}
