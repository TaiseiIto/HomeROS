use {
    super::{Directories, Directory},
    core::{
        fmt::{Debug, DebugStruct, Formatter, Result},
        mem::{offset_of, size_of_val},
    },
};

pub struct WithSize<'a> {
    directories: &'a Directories,
    size: usize,
}

impl<'a> WithSize<'a> {
    pub fn new(directories: &'a Directories, size: usize) -> Self {
        Self { directories, size }
    }

    fn export(&self) -> Option<&Directory> {
        (offset_of!(Directories, export) + size_of_val(&self.directories.export) <= self.size)
            .then_some(&self.directories.export)
            .and_then(Directory::filter)
    }

    fn import(&self) -> Option<&Directory> {
        (offset_of!(Directories, import) + size_of_val(&self.directories.import) <= self.size)
            .then_some(&self.directories.import)
            .and_then(Directory::filter)
    }

    fn resource(&self) -> Option<&Directory> {
        (offset_of!(Directories, resource) + size_of_val(&self.directories.resource) <= self.size)
            .then_some(&self.directories.resource)
            .and_then(Directory::filter)
    }

    fn exception(&self) -> Option<&Directory> {
        (offset_of!(Directories, exception) + size_of_val(&self.directories.exception) <= self.size)
            .then_some(&self.directories.exception)
            .and_then(Directory::filter)
    }

    fn security(&self) -> Option<&Directory> {
        (offset_of!(Directories, security) + size_of_val(&self.directories.security) <= self.size)
            .then_some(&self.directories.security)
            .and_then(Directory::filter)
    }

    fn base_relocation_table(&self) -> Option<&Directory> {
        (offset_of!(Directories, base_relocation_table)
            + size_of_val(&self.directories.base_relocation_table)
            <= self.size)
            .then_some(&self.directories.base_relocation_table)
            .and_then(Directory::filter)
    }

    fn debug(&self) -> Option<&Directory> {
        (offset_of!(Directories, debug) + size_of_val(&self.directories.debug) <= self.size)
            .then_some(&self.directories.debug)
            .and_then(Directory::filter)
    }

    fn architecture_specific_data(&self) -> Option<&Directory> {
        (offset_of!(Directories, architecture_specific_data)
            + size_of_val(&self.directories.architecture_specific_data)
            <= self.size)
            .then_some(&self.directories.architecture_specific_data)
            .and_then(Directory::filter)
    }

    fn rva_of_global_ptr(&self) -> Option<&Directory> {
        (offset_of!(Directories, rva_of_global_ptr)
            + size_of_val(&self.directories.rva_of_global_ptr)
            <= self.size)
            .then_some(&self.directories.rva_of_global_ptr)
            .and_then(Directory::filter)
    }

    fn tls(&self) -> Option<&Directory> {
        (offset_of!(Directories, tls) + size_of_val(&self.directories.tls) <= self.size)
            .then_some(&self.directories.tls)
            .and_then(Directory::filter)
    }

    fn load_configuration(&self) -> Option<&Directory> {
        (offset_of!(Directories, load_configuration)
            + size_of_val(&self.directories.load_configuration)
            <= self.size)
            .then_some(&self.directories.load_configuration)
            .and_then(Directory::filter)
    }

    fn bound_import(&self) -> Option<&Directory> {
        (offset_of!(Directories, bound_import) + size_of_val(&self.directories.bound_import)
            <= self.size)
            .then_some(&self.directories.bound_import)
            .and_then(Directory::filter)
    }

    fn import_address_table(&self) -> Option<&Directory> {
        (offset_of!(Directories, import_address_table)
            + size_of_val(&self.directories.import_address_table)
            <= self.size)
            .then_some(&self.directories.import_address_table)
            .and_then(Directory::filter)
    }

    fn delay_load_import(&self) -> Option<&Directory> {
        (offset_of!(Directories, delay_load_import)
            + size_of_val(&self.directories.delay_load_import)
            <= self.size)
            .then_some(&self.directories.delay_load_import)
            .and_then(Directory::filter)
    }

    fn com_runtime_descriptor(&self) -> Option<&Directory> {
        (offset_of!(Directories, com_runtime_descriptor)
            + size_of_val(&self.directories.com_runtime_descriptor)
            <= self.size)
            .then_some(&self.directories.com_runtime_descriptor)
            .and_then(Directory::filter)
    }
}

impl Debug for WithSize<'_> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        let mut debug_struct: DebugStruct = formatter.debug_struct("Directories");
        if let Some(export) = self.export() {
            debug_struct.field("export", &export);
        }
        if let Some(import) = self.import() {
            debug_struct.field("import", &import);
        }
        if let Some(resource) = self.resource() {
            debug_struct.field("resource", &resource);
        }
        if let Some(exception) = self.exception() {
            debug_struct.field("exception", &exception);
        }
        if let Some(security) = self.security() {
            debug_struct.field("security", &security);
        }
        if let Some(base_relocation_table) = self.base_relocation_table() {
            debug_struct.field("base_relocation_table", &base_relocation_table);
        }
        if let Some(debug) = self.debug() {
            debug_struct.field("debug", &debug);
        }
        if let Some(architecture_specific_data) = self.architecture_specific_data() {
            debug_struct.field("architecture_specific_data", &architecture_specific_data);
        }
        if let Some(rva_of_global_ptr) = self.rva_of_global_ptr() {
            debug_struct.field("rva_of_global_ptr", &rva_of_global_ptr);
        }
        if let Some(tls) = self.tls() {
            debug_struct.field("tls", &tls);
        }
        if let Some(load_configuration) = self.load_configuration() {
            debug_struct.field("load_configuration", &load_configuration);
        }
        if let Some(bound_import) = self.bound_import() {
            debug_struct.field("bound_import", &bound_import);
        }
        if let Some(import_address_table) = self.import_address_table() {
            debug_struct.field("import_address_table", &import_address_table);
        }
        if let Some(delay_load_import) = self.delay_load_import() {
            debug_struct.field("delay_load_import", &delay_load_import);
        }
        if let Some(com_runtime_descriptor) = self.com_runtime_descriptor() {
            debug_struct.field("com_runtime_descriptor", &com_runtime_descriptor);
        }
        debug_struct.finish()
    }
}
