mod v4;
mod v6;

use core::mem::ManuallyDrop;

/// # References
/// * [EFI_IP_ADDRESS](https://uefi.org/specs/UEFI/2.11/24_Network_Protocols_SNP_PXE_BIS.html#efi-pxe-base-code-protocol)
#[repr(C)]
pub union Address {
    v4: ManuallyDrop<v4::Address>,
    v6: ManuallyDrop<v6::Address>,
}

impl Address {
    pub fn v4(&self) -> &v4::Address {
        unsafe { &self.v4 }
    }

    pub fn v6(&self) -> &v6::Address {
        unsafe { &self.v6 }
    }
}
