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
            .and_then(|export| export.filter())
    }
}

impl Debug for WithSize<'_> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        let mut debug_struct: DebugStruct = formatter.debug_struct("Directories");
        if let Some(export) = self.export() {
            debug_struct.field("export", &export);
        }
        debug_struct.finish()
    }
}
