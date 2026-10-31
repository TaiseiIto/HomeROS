use {crate::Guid, guid::guid};

#[derive(Debug)]
#[repr(C)]
pub struct Table;

impl Table {
    pub const GUID: Guid = guid!("ee4e5898-3914-4259-9d6e-dc7bd79403cf");
}
