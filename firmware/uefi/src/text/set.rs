use crate::{Status, simple};

/// # References
/// * [EFI_TEXT_SET_MODE](https://uefi.org/specs/UEFI/2.11/12_Protocols_Console_Support.html#efi-simple-text-output-protocol-setmode)
pub type Mode = extern "efiapi" fn(*const simple::text::output::Protocol, usize) -> Status;
