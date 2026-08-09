use crate::{Char16, Status, simple};

pub mod set;

/// # References
/// * [EFI_TEXT_QUERY_MODE](https://uefi.org/specs/UEFI/2.11/12_Protocols_Console_Support.html#efi-simple-text-output-protocol-querymode)
pub type QueryMode = extern "efiapi" fn(
    *const simple::text::output::Protocol,
    usize,
    *mut usize,
    *mut usize,
) -> Status;

/// # References
/// * [EFI_TEXT_RESET](https://uefi.org/specs/UEFI/2.11/12_Protocols_Console_Support.html#efi-simple-text-output-protocol-reset)
pub type Reset = extern "efiapi" fn(*const simple::text::output::Protocol, bool) -> Status;

/// # References
/// * [EFI_TEXT_STRING](https://uefi.org/specs/UEFI/2.11/12_Protocols_Console_Support.html#efi-simple-text-output-protocol-outputstring)
pub type String =
    extern "efiapi" fn(*const simple::text::output::Protocol, *const Char16) -> Status;

/// # References
/// * [EFI_TEST_TEST_STRING](https://uefi.org/specs/UEFI/2.11/12_Protocols_Console_Support.html#efi-simple-text-output-protocol-teststring)
pub type TestString =
    extern "efiapi" fn(*const simple::text::output::Protocol, *const Char16) -> Status;
