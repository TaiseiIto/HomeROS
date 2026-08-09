use crate::{Char16, Status};

/// # References
/// * [SIMPLE_TEXT_OUTPUT_MODE](https://uefi.org/specs/UEFI/2.11/12_Protocols_Console_Support.html#efi-simple-text-output-protocol)
#[repr(C)]
pub struct Mode {
    max_mode: i32,
    mode: i32,
    attribute: i32,
    cusor_column: i32,
    cursor_row: i32,
    cursor_visible: bool,
}

/// # References
/// * [EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL](https://uefi.org/specs/UEFI/2.11/12_Protocols_Console_Support.html#efi-simple-text-output-protocol)
#[repr(C)]
pub struct Protocol {
    reset: Reset,
    output_string: String,
    test_string: TestString,
    query_mode: QueryMode,
    set_mode: SetMode,
    set_attribute: SetAttribute,
    clear_screen: ClearScreen,
    set_cursor_position: SetCursorPosition,
    enable_cursor: EnableCursor,
    mode: *const Mode,
}

/// # References
/// * [EFI_TEXT_CLEAR_SCREEN](https://uefi.org/specs/UEFI/2.11/12_Protocols_Console_Support.html#efi-simple-text-output-protocol-clearscreen)
pub type ClearScreen = extern "efiapi" fn(*const Protocol) -> Status;

/// # References
/// * [EFI_TEXT_ENABLE_CURSOR](https://uefi.org/specs/UEFI/2.11/12_Protocols_Console_Support.html#efi-simple-text-output-protocol-enablecursor)
pub type EnableCursor = extern "efiapi" fn(*const Protocol, bool) -> Status;

/// # References
/// * [EFI_TEXT_QUERY_MODE](https://uefi.org/specs/UEFI/2.11/12_Protocols_Console_Support.html#efi-simple-text-output-protocol-querymode)
pub type QueryMode = extern "efiapi" fn(*const Protocol, usize, *mut usize, *mut usize) -> Status;

/// # References
/// * [EFI_TEXT_RESET](https://uefi.org/specs/UEFI/2.11/12_Protocols_Console_Support.html#efi-simple-text-output-protocol-reset)
pub type Reset = extern "efiapi" fn(*const Protocol, bool) -> Status;

/// # References
/// * [EFI_TEXT_SET_ATTRIBUTE](https://uefi.org/specs/UEFI/2.11/12_Protocols_Console_Support.html#efi-simple-text-output-protocol-setattribute)
pub type SetAttribute = extern "efiapi" fn(*const Protocol, usize) -> Status;

/// # References
/// * [EFI_TEXT_SET_CURSOR_POSITION](https://uefi.org/specs/UEFI/2.11/12_Protocols_Console_Support.html#efi-simple-text-output-protocol-setcursorposition)
pub type SetCursorPosition = extern "efiapi" fn(*const Protocol, usize, usize) -> Status;

/// # References
/// * [EFI_TEXT_SET_MODE](https://uefi.org/specs/UEFI/2.11/12_Protocols_Console_Support.html#efi-simple-text-output-protocol-setmode)
pub type SetMode = extern "efiapi" fn(*const Protocol, usize) -> Status;
/// # References
/// * [EFI_TEXT_STRING](https://uefi.org/specs/UEFI/2.11/12_Protocols_Console_Support.html#efi-simple-text-output-protocol-outputstring)
pub type String = extern "efiapi" fn(*const Protocol, *const Char16) -> Status;

/// # References
/// * [EFI_TEST_TEST_STRING](https://uefi.org/specs/UEFI/2.11/12_Protocols_Console_Support.html#efi-simple-text-output-protocol-teststring)
pub type TestString = extern "efiapi" fn(*const Protocol, *const Char16) -> Status;
