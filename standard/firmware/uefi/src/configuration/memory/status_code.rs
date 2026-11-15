use {
    super::super::Table,
    crate::{Guid, guid},
};

#[derive(Debug)]
#[repr(C)]
pub struct Record;

impl Record {
    /// # References
    /// * [MEMORY_STATUS_CODE_RECORD_GUID](https://android.googlesource.com/platform/external/rust/crates/uefi/+/refs/heads/upstream/src/table/cfg.rs#85)
    pub const GUID: Guid = guid!("060cc026-4c0d-4dda-8f41-595fef00a502");
}

impl Table for Record {}
