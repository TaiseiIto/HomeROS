use crate::{Event, Status};

/// References
/// * [SetTimer](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-settimer)
#[must_use]
pub type Set = extern "efiapi" fn(Event, Delay, u64) -> Status;

/// References
/// * [EFI_TIMER_DELAY](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-settimer)
#[repr(C)]
pub enum Delay {
    Cancel,
    Periodic,
    Relative,
}
