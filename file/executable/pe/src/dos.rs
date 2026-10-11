use {
    alloc::string::{String, ToString},
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [DOS Header](https://0xrick.github.io/win-internals/pe3/#dos-header)
#[repr(C)]
pub struct Header {
    magic: u16,
    cblp: u16,
    cp: u16,
    crlc: u16,
    cparhdr: u16,
    minalloc: u16,
    maxalloc: u16,
    ss: u16,
    sp: u16,
    csum: u16,
    ip: u16,
    cs: u16,
    lfarlc: u16,
    ovno: u16,
    res: [u16; 4],
    oemid: u16,
    oeminfo: u16,
    res2: [u16; 10],
    lfanew: u16,
}

impl Header {
    fn magic(&self) -> String {
        str::from_utf8(self.magic.to_le_bytes().as_slice())
            .unwrap()
            .to_string()
    }
}

impl Debug for Header {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Header")
            .field("magic", &self.magic())
            .field("cblp", &self.cblp)
            .field("cp", &self.cp)
            .field("crlc", &self.crlc)
            .field("cparhdr", &self.cparhdr)
            .field("minalloc", &self.minalloc)
            .field("maxalloc", &self.maxalloc)
            .field("ss", &self.ss)
            .field("sp", &self.sp)
            .field("csum", &self.csum)
            .field("ip", &self.ip)
            .field("cs", &self.cs)
            .field("lfarlc", &self.lfarlc)
            .field("ovno", &self.ovno)
            .field("res", &self.res)
            .field("oemid", &self.oemid)
            .field("oeminfo", &self.oeminfo)
            .field("res2", &self.res2)
            .field("lfanew", &self.lfanew)
            .finish()
    }
}
