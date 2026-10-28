use super::{Node, Type};

/// # References
/// * [ACPI Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#acpi-device-path)
/// # TODO
/// * Print all subtypes.
#[derive(Debug)]
#[repr(C)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
}

impl Node for Protocol {}

/// # References
/// * [ACPI Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#acpi-device-path)
#[derive(Debug)]
#[repr(u8)]
enum SubType {
    Acpi = 1,
    ExpandedAcpi = 2,
    Adr = 3,
    Nvdimm = 4,
}
