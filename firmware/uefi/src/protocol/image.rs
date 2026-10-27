use {
    super::device_path,
    crate::{
        Handle, Void,
        service::boot::{image::Unload, memory::Type},
        system,
    },
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [EFI_LOADED_IMAGE_PROTOCOL](https://uefi.org/specs/UEFI/2.11/09_Protocols_EFI_Loaded_Image.html#id3)
#[repr(C)]
pub struct Protocol {
    revision: u32,
    parent: Handle,
    system_table: *const system::Table,
    device: Handle,
    file_path: *const device_path::Protocol,
    reserved: *const Void,
    load_options_size: u32,
    load_options: *const Void,
    image_base: *const Void,
    image_size: u64,
    image_code: Type,
    image_data: Type,
    unload: Unload,
}

impl Debug for Protocol {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Protocol")
            .field("revision", &self.revision)
            .field("parent", &self.parent)
            .field("system_table", &self.system_table)
            .field("device", &self.device)
            .field("file_path", unsafe { &*self.file_path })
            .field("load_options_size", &self.load_options_size)
            .field("load_options", &self.load_options)
            .field("image_base", &self.image_base)
            .field("image_size", &self.image_size)
            .field("image_code", &self.image_code)
            .field("image_data", &self.image_data)
            .field("unload", &self.unload)
            .finish()
    }
}
