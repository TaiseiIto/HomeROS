use {
    crate::Header,
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [SMBIOS Specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.10.0.pdf) 7.2 System Information (Type 1)
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Table {
    header: Header,
}

impl Debug for Table {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Table")
            .field("header", &self.header)
            .finish()
    }
}

impl crate::Table for Table {}
