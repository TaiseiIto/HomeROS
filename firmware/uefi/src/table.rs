/// # References
/// * [EFI_TABLE_HEADER](https://uefi.org/specs/UEFI/2.11/04_EFI_System_Table.html#id4)
#[derive(Debug)]
#[repr(C)]
pub struct Header {
    signature: u64,
    revision: u32,
    header_size: u32,
    crc32: u32,
    __: u32,
}
