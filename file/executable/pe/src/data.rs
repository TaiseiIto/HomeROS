/// # References
/// * [Data Directories](https://0xrick.github.io/win-internals/pe5/#data-directories)
#[derive(Debug)]
#[repr(C)]
pub struct Directories {
    export: Directory,
    import: Directory,
    resource: Directory,
    exception: Directory,
    security: Directory,
    base_relocation_table: Directory,
    debug: Directory,
    architecture_specific_data: Directory,
    rva_of_global_ptr: Directory,
    tls: Directory,
    load_configuration: Directory,
    bound_import: Directory,
    import_address_table: Directory,
    delay_load_import: Directory,
    com_runtime_descriptor: Directory,
}

#[derive(Debug)]
#[repr(C)]
struct Directory {
    virtual_address: u32,
    size: u32,
}
