use core::fmt::{Debug, Formatter, Result};

/// # References
/// * [Firmware ACPI Control Structure (FACS)](https://uefi.org/htmlspecs/ACPI_Spec_6_4_html/05_ACPI_Software_Programming_Model/ACPI_Software_Programming_Model.html#firmware-acpi-control-structure-facs)
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Structure {
    signature: [u8; 4],
    length: u32,
    hardware_signature: u32,
    firmware_waking_vector: u32,
    global_lock: u32,
    flags: FlagsRaw,
    x_firmware_waking_vector: u64,
    version: u8,
    reserved0: [u8; 3],
    ospm_flags: OspmFlagsRaw,
    reserved1: [u8; 24],
}

impl Structure {
    fn get_flags(&self) -> Flags {
        self.flags().into()
    }

    fn get_ospm_flags(&self) -> OspmFlags {
        self.ospm_flags().into()
    }

    fn get_signature(&self) -> &str {
        str::from_utf8(&self.signature).unwrap()
    }
}

impl Debug for Structure {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Structure")
            .field("signature", &self.get_signature())
            .field("length", &self.length())
            .field("hardware_signature", &self.hardware_signature())
            .field("firmware_waking_vector", &self.firmware_waking_vector())
            .field("global_lock", &self.global_lock())
            .field("flags", &self.get_flags())
            .field("x_firmware_waking_vector", &self.x_firmware_waking_vector())
            .field("version", &self.version())
            .field("ospm_flags", &self.get_ospm_flags())
            .finish()
    }
}

/// # References
/// * [Firmware Control Structure Feature Flags](https://uefi.org/htmlspecs/ACPI_Spec_6_4_html/05_ACPI_Software_Programming_Model/ACPI_Software_Programming_Model.html?highlight=hardware_reduced_acpi#firmware-control-structure-feature-flags)
#[io::register]
struct Flags {
    s4bios_f: bool,
    bit64_wake_supported_f: bool,
    __: [bool; 30],
}

/// # References
/// * [OSPM Enabled Firmware Control Structure Feature Flags](https://uefi.org/htmlspecs/ACPI_Spec_6_4_html/05_ACPI_Software_Programming_Model/ACPI_Software_Programming_Model.html?highlight=hardware_reduced_acpi#ospm-enabled-firmware-control-structure-feature-flags)
#[io::register]
struct OspmFlags {
    bit64_wake_f: bool,
    __: [bool; 31],
}
