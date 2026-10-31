use {
    crate::{Guid, Handle, protocol::image::Protocol},
    core::fmt::{Debug, Formatter, Result},
    guid::guid,
};

/// # References
/// * [EFI_DEBUG_IMAGE_INFO_TABLE_HEADER](https://uefi.org/specs/UEFI/2.11/18_Protocols_Debugger_Support.html#efi-image-info)
#[repr(C)]
pub struct Table {
    update_status: UpdateStatusRaw,
    table_size: u32,
    info: *const Info,
}

impl Table {
    pub const GUID: Guid = guid!("49152e77-1ada-4764-b7a2-7afefed95e8b");
}

impl Debug for Table {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Table")
            .field("update_status", &self.update_status)
            .field("table_size", &self.table_size)
            .field("info", unsafe { &*self.info })
            .finish()
    }
}

#[io::register]
struct UpdateStatus {
    update_in_progress: bool,
    modified: bool,
    __: [bool; 30],
}

/// # References
/// * [EFI_DEBUG_IMAGE_INFO](https://uefi.org/specs/UEFI/2.11/18_Protocols_Debugger_Support.html#efi-image-info)
#[repr(C)]
struct Info {
    ty: *const u32,
    normal: *const Normal,
}

impl Debug for Info {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Info")
            .field("ty", unsafe { &*self.ty })
            .field("normal", unsafe { &*self.normal })
            .finish()
    }
}

/// # References
/// * [EFI_DEBUG_IMAGE_INFO_NORMAL](https://uefi.org/specs/UEFI/2.11/18_Protocols_Debugger_Support.html#efi-image-info)
#[repr(C)]
struct Normal {
    ty: u32,
    loaded_image: *const Protocol,
    image: Handle,
}

impl Debug for Normal {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Normal")
            .field("ty", &self.ty)
            .field("loaded_image", unsafe { &*self.loaded_image })
            .field("image", &self.image)
            .finish()
    }
}
