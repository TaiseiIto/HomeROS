use crate::{Char16, Event, Status};

/// # References
/// * [EFI_SIMPLE_TEXT_INPUT_PROTOCOL](https://uefi.org/specs/UEFI/2.11/12_Protocols_Console_Support.html#efi-simple-text-input-protocol)
#[repr(C)]
pub struct Protocol {
    reset: Reset,
    read_key_stroke: ReadKey,
    wait_for_key: Event,
}

/// References
/// * [EFI_INPUT_KEY](https://uefi.org/specs/UEFI/2.11/12_Protocols_Console_Support.html#efi-simple-text-input-protocol-readkeystroke)
#[repr(C)]
pub struct Key {
    scan_code: u16,
    unicode_char: Char16,
}

/// References
/// * [EFI_INPUT_READ_KEY](https://uefi.org/specs/UEFI/2.11/12_Protocols_Console_Support.html#efi-simple-text-input-protocol-readkeystroke)
pub type ReadKey = extern "efiapi" fn(*const Protocol, *mut Key) -> Status;

/// References
/// * [EFI_INPUT_RESET](https://uefi.org/specs/UEFI/2.11/12_Protocols_Console_Support.html#efi-simple-text-input-protocol-reset)
pub type Reset = extern "efiapi" fn(*const Protocol, bool) -> Status;
