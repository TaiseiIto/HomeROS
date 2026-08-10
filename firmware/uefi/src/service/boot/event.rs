use {
    super::task::Priority,
    crate::{Event, Guid, Status, Void},
};

/// References
/// * [CheckEvent](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-checkevent)
pub type Check = extern "efiapi" fn(Event) -> Status;

/// References
/// * [CloseEvent](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-closeevent)
pub type Close = extern "efiapi" fn(Event) -> Status;

/// References
/// * [CreateEvent](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-createevent)
pub type Create = extern "efiapi" fn(u32, Priority, Notify, *const Void, *mut Event) -> Status;

/// References
/// * [CreateEventEx](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-createeventex)
pub type CreateEx =
    extern "efiapi" fn(u32, Priority, Notify, *const Void, *const Guid, *mut Event) -> Status;

/// References
/// * [EFI_EVENT_NOTIFY](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-createevent)
pub type Notify = extern "efiapi" fn(Event, *const Void) -> Void;

/// References
/// * [SignalEvent](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-signalevent)
pub type Signal = extern "efiapi" fn(Event) -> Status;

/// References
/// * [WaitForEvent](https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-waitforevent)
pub type Wait = extern "efiapi" fn(usize, *const Event, *mut usize) -> Status;
