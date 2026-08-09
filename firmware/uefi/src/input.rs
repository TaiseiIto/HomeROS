use crate::{Char16, Status, simple};

/// References
/// * [EFI_INPUT_KEY](https://uefi.org/specs/UEFI/2.11/12_Protocols_Console_Support.html#efi-simple-text-input-protocol-readkeystroke)
#[repr(C)]
pub struct Key {
    scan_code: u16,
    unicode_char: Char16,
}

/// References
/// * [EFI_INPUT_READ_KEY](https://uefi.org/specs/UEFI/2.11/12_Protocols_Console_Support.html#efi-simple-text-input-protocol-readkeystroke)
pub type ReadKey = extern "efiapi" fn(*const simple::text::input::Protocol, *mut Key) -> Status;

/// References
/// * [EFI_INPUT_RESET](https://uefi.org/specs/UEFI/2.11/12_Protocols_Console_Support.html#efi-simple-text-input-protocol-reset)
pub type Reset = extern "efiapi" fn(*const simple::text::input::Protocol, bool) -> Status;
