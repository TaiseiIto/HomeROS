/// # References
/// * [EFI_LOADED_IMAGE_PROTOCOL](https://uefi.org/specs/UEFI/2.11/09_Protocols_EFI_Loaded_Image.html#id3)
/// # TODO
/// * Add all fields.
#[derive(Debug)]
#[repr(C)]
pub struct Protocol {
    revision: u32,
}
