use crate::{Status, simple};

/// # References
/// * [EFI_TEXT_RESET](https://uefi.org/specs/UEFI/2.11/12_Protocols_Console_Support.html#efi-simple-text-output-protocol-reset)
pub type Reset = extern "efiapi" fn(*const simple::text::output::Protocol, bool) -> Status;
