#![no_std]

mod configuration;
mod protocol;
mod service;
pub mod system;
mod table;

/// # References
/// * [EFI_GUID](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-installprotocolinterface)
#[repr(C)]
pub struct Guid {
    data1: u32,
    data2: u16,
    data3: u16,
    data4: [u8; 8],
}

/// # References
/// * [CHAR16](https://uefi.org/specs/UEFI/2.11/02_Overview.html#data-types)
pub type Char16 = u16;

/// # References
/// * [EFI_EVENT](https://uefi.org/specs/UEFI/2.11/02_Overview.html#data-types)
/// * [EFI_EVENT](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-createevent)
pub type Event = *const Void;

/// # References
/// * [EFI_HANDLE](https://uefi.org/specs/UEFI/2.11/02_Overview.html#data-types)
/// * [EFI_HANDLE](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-installprotocolinterface)
pub type Handle = *const Void;

/// # References
/// * [EFI_STATUS](https://uefi.org/specs/UEFI/2.11/02_Overview.html#data-types)
pub type Status = usize;

/// # References
/// * [VOID](https://uefi.org/specs/UEFI/2.11/02_Overview.html#data-types)
pub type Void = ();
