use crate::input;

/// # References
/// * [EFI_SIMPLE_TEXT_INPUT_PROTOCOL](https://uefi.org/specs/UEFI/2.11/12_Protocols_Console_Support.html#efi-simple-text-input-protocol)
#[repr(C)]
pub struct Protocol {
    reset: input::Reset,
    read_key_stroke: input::ReadKey,
}
