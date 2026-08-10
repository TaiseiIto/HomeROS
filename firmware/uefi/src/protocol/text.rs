mod input;
mod output;

use crate::Event;

/// # References
/// * [EFI_SIMPLE_TEXT_INPUT_PROTOCOL](https://uefi.org/specs/UEFI/2.11/12_Protocols_Console_Support.html#efi-simple-text-input-protocol)
#[repr(C)]
pub struct Input {
    reset: input::Reset,
    read_key_stroke: input::ReadKey,
    wait_for_key: Event,
}

/// # References
/// * [EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL](https://uefi.org/specs/UEFI/2.11/12_Protocols_Console_Support.html#efi-simple-text-output-protocol)
#[repr(C)]
pub struct Output {
    reset: output::Reset,
    output_string: output::String,
    test_string: output::TestString,
    query_mode: output::QueryMode,
    set_mode: output::SetMode,
    set_attribute: output::SetAttribute,
    clear_screen: output::ClearScreen,
    set_cursor_position: output::SetCursorPosition,
    enable_cursor: output::EnableCursor,
    mode: *const output::Mode,
}
