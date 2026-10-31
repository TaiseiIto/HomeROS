use {
    alloc::{
        string::{String, ToString},
        vec::Vec,
    },
    core::fmt::{Debug, Display, Formatter, Result},
};

/// # References
/// * [EFI_IPv4_ADDRESS](https://uefi.org/specs/UEFI/2.11/24_Network_Protocols_SNP_PXE_BIS.html#efi-pxe-base-code-protocol)
#[repr(C)]
pub struct Address([u8; 4]);

impl Debug for Address {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        Display::fmt(self, formatter)
    }
}

impl Display for Address {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        let address: Vec<String> = self.0.as_slice().iter().map(ToString::to_string).collect();
        Display::fmt(&address.join(":"), formatter)
    }
}
