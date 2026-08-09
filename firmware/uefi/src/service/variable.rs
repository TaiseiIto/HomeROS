use crate::{Char16, Guid, Status, Void};

/// # References
/// * [GetVariable](https://uefi.org/specs/UEFI/2.11/08_Services_Runtime_Services.html#getvariable)
pub type Get =
    extern "efiapi" fn(*const Char16, *const Guid, *mut u32, *mut usize, *mut Void) -> Status;
