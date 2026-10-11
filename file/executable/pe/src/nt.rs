use {
    alloc::string::String,
    core::fmt::{Debug, Formatter, Result},
};

pub struct Header {
    signature: u32,
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
            .finish()
    }
}
