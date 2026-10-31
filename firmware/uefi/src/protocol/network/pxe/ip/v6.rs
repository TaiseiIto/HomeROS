use {
    alloc::vec::Vec,
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [EFI_IPv6_Address](https://uefi.org/specs/UEFI/2.11/24_Network_Protocols_SNP_PXE_BIS.html#efi-pxe-base-code-protocol)
#[repr(C)]
pub struct Address([u8; 16]);

impl Address {
    fn words(&self) -> [u16; 8] {
        let words: Vec<u16> = self
            .0
            .as_slice()
            .chunks(2)
            .map(|word| u16::from_le_bytes(word.to_vec().try_into().unwrap()))
            .collect();
        words.try_into().unwrap()
    }
}
