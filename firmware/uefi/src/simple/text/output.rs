use crate::text;

/// # References
/// * [EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL](https://uefi.org/specs/UEFI/2.11/12_Protocols_Console_Support.html#efi-simple-text-output-protocol)
#[repr(C)]
pub struct Protocol {
    reset: text::Reset,
    output_string: text::String,
    test_string: text::TestString,
    query_mode: text::QueryMode,
    set_mode: text::SetMode,
}
