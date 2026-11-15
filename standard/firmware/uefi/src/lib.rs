#![no_std]
#![feature(iter_array_chunks)]
#![feature(ascii_char)]

extern crate alloc;

mod acpi;
mod configuration;
mod pi;
mod protocol;
pub mod service;
pub mod system;
mod table;

use uuid_from_str::uuid as guid;

/// # References
/// * [EFI_GUID](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-installprotocolinterface)
type Guid = uuid::Uuid;

/// # References
/// * [EFI_STATUS](https://uefi.org/specs/UEFI/2.11/02_Overview.html#data-types)
#[derive(Debug, Eq, PartialEq)]
#[must_use]
#[repr(transparent)]
pub struct Status(usize);

impl Status {
    /// # References
    /// * [Status Codes](https://uefi.org/specs/UEFI/2.11/Apx_D_Status_Codes.html)
    const SUCCESS: Self = Self(0);
    const BUFFER_TOO_SMALL: Self = Self((1 << (usize::BITS - 1)) + 5);

    pub fn assert(self) {
        assert_eq!(self, Self::SUCCESS);
    }

    pub fn assert_buffer_too_small(self) {
        assert_eq!(self, Self::BUFFER_TOO_SMALL);
    }
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
/// * [EFI_HANDLE](https://uefi.org/specs/UEFI/2.11/02_Overview.html#data-types)
/// * [EFI_HANDLE](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-installprotocolinterface)
pub type HandleMut = *mut Void;

/// # References
/// * [VOID](https://uefi.org/specs/UEFI/2.11/02_Overview.html#data-types)
pub type Void = ();
