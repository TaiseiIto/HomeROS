/// # References
/// * [Data Directories](https://0xrick.github.io/win-internals/pe5/#data-directories)
#[derive(Debug)]
#[repr(C)]
pub struct Directory {
    virtual_address: u32,
    size: u32,
}
