use {
    crate::{file, optional, section},
    alloc::string::String,
    core::{
        fmt::{Debug, Formatter, Result},
        mem::offset_of,
        slice::from_raw_parts,
    },
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
    pub fn section_table(&self) -> &[section::Header] {
        let header: *const Self = self as *const Self;
        let offset: usize = offset_of!(Self, optional_header) + self.size_of_optional_header();
        let section_table: *const section::Header =
            unsafe { header.byte_add(offset) } as *const section::Header;
        let number_of_sections: usize = self.number_of_sections();
        unsafe { from_raw_parts(section_table, number_of_sections) }
    }

    fn number_of_sections(&self) -> usize {
        self.file_header.number_of_sections()
    }

    fn optional_header<'a>(&'a self) -> optional::header::WithSize<'a> {
        self.optional_header
            .with_size(self.size_of_optional_header())
    }

    fn signature(&self) -> String {
        str::from_utf8(self.signature.to_le_bytes().as_slice())
            .unwrap()
            .chars()
            .take_while(|character| *character != '\0')
            .collect()
    }

    fn size_of_optional_header(&self) -> usize {
        self.file_header.size_of_optional_header()
    }
}

impl Debug for Header {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Header")
            .field("signature", &self.signature())
            .field("file_header", &self.file_header)
            .field("optional_header", &self.optional_header())
            .finish()
    }
}
