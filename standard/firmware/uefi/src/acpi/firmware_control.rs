use core::fmt::{Debug, Formatter, Result};

/// # Reference
/// * [Firmware ACPI Control Structure (FACS)](https://uefi.org/htmlspecs/ACPI_Spec_6_4_html/05_ACPI_Software_Programming_Model/ACPI_Software_Programming_Model.html#firmware-acpi-control-structure-facs)
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Structure {
    signature: [u8; 4],
    length: u32,
    hardware_signature: u32,
    firmware_waking_vector: u32,
    global_lock: u32,
    flags: u32,
    x_firmware_waking_vector: u64,
    version: u8,
    reserved0: [u8; 3],
    ospm_flags: u32,
    reserved1: [u8; 24],
}

impl Debug for Structure {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Structure")
            .field("signature", &self.signature())
            .field("length", &self.length())
            .field("hardware_signature", &self.hardware_signature())
            .field("firmware_waking_vector", &self.firmware_waking_vector())
            .field("global_lock", &self.global_lock())
            .field("flags", &self.flags())
            .field("x_firmware_waking_vector", &self.x_firmware_waking_vector())
            .field("version", &self.version())
            .field("ospm_flags", &self.ospm_flags())
            .finish()
    }
}
