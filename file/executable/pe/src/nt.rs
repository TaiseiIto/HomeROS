use {
    alloc::string::{String, ToString},
    core::fmt::{Debug, Formatter, Result},
};

pub struct Header {
    signature: u32,
}

impl Header {
    fn signature(&self) -> String {
        str::from_utf8(self.signature.to_le_bytes().as_slice())
            .unwrap()
            .to_string()
    }
}

impl Debug for Header {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Header")
            .field("signature", &self.signature())
            .finish()
    }
}
