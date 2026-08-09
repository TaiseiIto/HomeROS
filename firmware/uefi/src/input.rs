use crate::{Status, simple};

/// References
/// * [EFI_INPUT_RESET](https://uefi.org/specs/UEFI/2.11/12_Protocols_Console_Support.html#efi-simple-text-input-protocol-reset)
pub type Reset = extern "efiapi" fn(*const simple::text::input::Protocol, bool) -> Status;
