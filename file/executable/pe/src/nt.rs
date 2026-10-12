use {
    crate::{file, optional},
    alloc::string::String,
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [NT Headers (IMAGE_NT_HEADERS)](https://0xrick.github.io/win-internals/pe4/#nt-headers-image_nt_headers)
#[repr(C)]
pub struct Header {
    signature: u32,
    file_header: file::Header,
    optional_header: optional::Header,
}

impl Header {
    fn signature(&self) -> String {
        str::from_utf8(self.signature.to_le_bytes().as_slice())
            .unwrap()
            .chars()
            .take_while(|character| *character != '\0')
            .collect()
    }
}

impl Debug for Header {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Header")
            .field("signature", &self.signature())
            .field("file_header", &self.file_header)
            .finish()
    }
}
