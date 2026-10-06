use core::{
    fmt::{Debug, Formatter, Result},
    mem::{offset_of, size_of, size_of_val},
    num::Wrapping,
    slice::from_raw_parts,
};

/// # References
/// * [Root System Description Pointer (RSDP) Structure](https://uefi.org/htmlspecs/ACPI_Spec_6_4_html/05_ACPI_Software_Programming_Model/ACPI_Software_Programming_Model.html#rsdp-structure)
/// # TODO
/// * Print RSDT
/// * Print XSDT
#[repr(C)]
pub struct Pointer {
    signature: [u8; 8],
    checksum: u8,
    oemid: [u8; 6],
    revision: u8,
    rsdt: u32,
    length: u32,
    xsdt: u64,
    extended_checksum: u8,
    __: [u8; 3],
}

impl Pointer {
    pub fn is_correct(&self) -> bool {
        let pointer: *const Self = self as *const Self;
        let pointer: *const u8 = pointer as *const u8;
        unsafe { from_raw_parts(pointer, offset_of!(Self, length)) }
            .iter()
            .copied()
            .map(Wrapping)
            .sum::<Wrapping<u8>>()
            .0
            == 0
            && unsafe { from_raw_parts(pointer, self.length as usize) }
                .iter()
                .copied()
                .map(Wrapping)
                .sum::<Wrapping<u8>>()
                .0
                == 0
    }

    fn signature(&self) -> &str {
        str::from_utf8(&self.signature).unwrap()
    }

    fn oemid(&self) -> &str {
        str::from_utf8(&self.oemid).unwrap()
    }
}

impl Debug for Pointer {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Pointer")
            .field("signature", &self.signature())
            .field("oemid", &self.oemid())
            .field("revision", &self.revision)
            .field("rsdt", &self.rsdt)
            .field("xsdt", &self.xsdt)
            .finish()
    }
}
