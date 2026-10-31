use core::fmt::{Debug, Formatter, Result};

/// # References
/// * [EFI_IPv4_Address](https://uefi.org/specs/UEFI/2.11/24_Network_Protocols_SNP_PXE_BIS.html#efi-pxe-base-code-protocol)
#[repr(C)]
pub struct Address([u8; 4]);
