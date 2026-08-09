use crate::text;

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
    reset: text::Reset,
    output_string: text::String,
    test_string: text::TestString,
    query_mode: text::QueryMode,
    set_mode: text::set::Mode,
    set_attribute: text::set::Attribute,
    clear_screen: text::ClearScreen,
    set_cursor_position: text::set::CursorPosition,
    enable_cursor: text::EnableCursor,
    mode: *const Mode,
}
