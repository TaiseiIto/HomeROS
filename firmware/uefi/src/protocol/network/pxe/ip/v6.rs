use {
    alloc::{
        string::{String, ToString},
        vec::Vec,
    },
    core::fmt::{Debug, Display, Formatter, Result},
};

/// # References
/// * [EFI_IPv6_ADDRESS](https://uefi.org/specs/UEFI/2.11/24_Network_Protocols_SNP_PXE_BIS.html#efi-pxe-base-code-protocol)
#[derive(Clone, Default)]
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

impl Debug for Address {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        Display::fmt(self, formatter)
    }
}

impl Display for Address {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        let words: Vec<String> = self
            .words()
            .as_slice()
            .iter()
            .map(ToString::to_string)
            .collect();
        Display::fmt(&words.join(":"), formatter)
    }
}
