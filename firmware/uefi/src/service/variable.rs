use crate::{Char16, Guid, Status, Void};

/// # References
/// * [GetVariable](https://uefi.org/specs/UEFI/2.11/08_Services_Runtime_Services.html#getvariable)
pub type Get =
    extern "efiapi" fn(*const Char16, *const Guid, *mut u32, *mut usize, *mut Void) -> Status;

/// # References
/// * [GetNextVariableName](https://uefi.org/specs/UEFI/2.11/08_Services_Runtime_Services.html#getnextvariablename)
pub type Next = extern "efiapi" fn(*mut usize, *mut Char16, *mut Guid) -> Status;

/// # References
/// * [SetVariable](https://uefi.org/specs/UEFI/2.11/08_Services_Runtime_Services.html#setvariable)
pub type Set = extern "efiapi" fn(*const Char16, *const Guid, u32, usize, *const Void) -> Status;
