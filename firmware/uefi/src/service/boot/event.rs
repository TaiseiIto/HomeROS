use {
    super::task::Priority,
    crate::{Event, Status, Void},
};

/// References
/// * [CreateEvent](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-createevent)
pub type Create = extern "efiapi" fn(u32, Priority, Notify, *const Void, *mut Event) -> Status;

/// References
/// * [EFI_EVENT_NOTIFY](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-createevent)
pub type Notify = extern "efiapi" fn(Event, *const Void) -> Void;
