use core::fmt::{Debug, Formatter, Result};

/// # References
/// * [EFI_TABLE_HEADER](https://uefi.org/specs/UEFI/2.11/04_EFI_System_Table.html#id4)
#[repr(C)]
pub struct Header {
    signature: u64,
    revision: u32,
    header_size: u32,
    crc32: u32,
    __: u32,
}

impl Debug for Header {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Header")
            .field(
                "signature",
                &str::from_utf8(self.signature.to_le_bytes().as_slice()).unwrap(),
            )
            .field("revision", &self.revision)
            .field("header_size", &self.header_size)
            .field("crc32", &self.crc32)
            .finish()
    }
}
