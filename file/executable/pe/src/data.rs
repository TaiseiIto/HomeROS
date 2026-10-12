use core::fmt::{Debug, Formatter, Result};

/// # References
/// * [Data Directories](https://0xrick.github.io/win-internals/pe5/#data-directories)
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
    null: Directory,
}

impl Debug for Directories {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_map()
            .entries(
                [
                    ("export", &self.export),
                    ("import", &self.import),
                    ("resource", &self.resource),
                    ("exception", &self.exception),
                    ("security", &self.security),
                    ("base_relocation_table", &self.base_relocation_table),
                    ("debug", &self.debug),
                    (
                        "architecture_specific_data",
                        &self.architecture_specific_data,
                    ),
                    ("rva_of_global_ptr", &self.rva_of_global_ptr),
                    ("tls", &self.tls),
                    ("load_configuration", &self.load_configuration),
                    ("bound_import", &self.bound_import),
                    ("import_address_table", &self.import_address_table),
                    ("delay_load_import", &self.delay_load_import),
                    ("com_runtime_descriptor", &self.com_runtime_descriptor),
                    ("null", &self.null),
                ]
                .as_slice()
                .iter()
                .filter_map(|(name, directory)| {
                    directory.filter().map(|directory| (name, directory))
                }),
            )
            .finish()
    }
}

#[derive(Debug)]
#[repr(C)]
struct Directory {
    virtual_address: u32,
    size: u32,
}

impl Directory {
    fn filter(&self) -> Option<&Self> {
        (self.virtual_address != 0 && self.size != 0).then_some(self)
    }
}
