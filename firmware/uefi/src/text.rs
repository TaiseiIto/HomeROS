use crate::{Char16, Status, simple};

/// # References
/// * [EFI_TEXT_RESET](https://uefi.org/specs/UEFI/2.11/12_Protocols_Console_Support.html#efi-simple-text-output-protocol-reset)
pub type Reset = extern "efiapi" fn(*const simple::text::output::Protocol, bool) -> Status;

/// # References
/// * [EFI_TEXT_STRING](https://uefi.org/specs/UEFI/2.11/12_Protocols_Console_Support.html#efi-simple-text-output-protocol-outputstring)
pub type String =
    extern "efiapi" fn(*const simple::text::output::Protocol, *const Char16) -> Status;
