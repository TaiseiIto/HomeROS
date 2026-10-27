use {
    super::device_path,
    crate::{
        Handle, Void,
        service::boot::{image::Unload, memory::Type},
        system,
    },
};

/// # References
/// * [EFI_LOADED_IMAGE_PROTOCOL](https://uefi.org/specs/UEFI/2.11/09_Protocols_EFI_Loaded_Image.html#id3)
/// # TODO
/// * Add all fields.
#[derive(Debug)]
#[repr(C)]
pub struct Protocol {
    revision: u32,
    parent: Handle,
    system_table: *const system::Table,
    device: Handle,
    file_path: device_path::Protocol,
    reserved: *const Void,
    load_options_size: u32,
    load_options: *const Void,
    image_base: *const Void,
    image_size: u64,
    image_code: Type,
    image_data: Type,
    unload: Unload,
}
