use {
    super::{Header, Subsystem},
    crate::data,
    core::{
        fmt::{Debug, DebugStruct, Formatter, Result},
        mem::{offset_of, size_of_val},
    },
};

pub struct WithSize<'a> {
    header: &'a Header,
    size: usize,
}

impl<'a> WithSize<'a> {
    pub fn new(header: &'a Header, size: usize) -> Self {
        Self { header, size }
    }

    fn magic(&self) -> Option<u16> {
        (offset_of!(Header, magic) + size_of_val(&self.header.magic) <= self.size)
            .then_some(self.header.magic)
    }

    fn major_linker_version(&self) -> Option<u8> {
        (offset_of!(Header, major_linker_version) + size_of_val(&self.header.major_linker_version)
            <= self.size)
            .then_some(self.header.major_linker_version)
    }

    fn minor_linker_version(&self) -> Option<u8> {
        (offset_of!(Header, minor_linker_version) + size_of_val(&self.header.minor_linker_version)
            <= self.size)
            .then_some(self.header.minor_linker_version)
    }

    fn size_of_code(&self) -> Option<u32> {
        (offset_of!(Header, size_of_code) + size_of_val(&self.header.size_of_code) <= self.size)
            .then_some(self.header.size_of_code)
    }

    fn size_of_initialized_data(&self) -> Option<u32> {
        (offset_of!(Header, size_of_initialized_data)
            + size_of_val(&self.header.size_of_initialized_data)
            <= self.size)
            .then_some(self.header.size_of_initialized_data)
    }

    fn size_of_uninitialized_data(&self) -> Option<u32> {
        (offset_of!(Header, size_of_uninitialized_data)
            + size_of_val(&self.header.size_of_uninitialized_data)
            <= self.size)
            .then_some(self.header.size_of_uninitialized_data)
    }

    fn address_of_entry_point(&self) -> Option<u32> {
        (offset_of!(Header, address_of_entry_point)
            + size_of_val(&self.header.address_of_entry_point)
            <= self.size)
            .then_some(self.header.address_of_entry_point)
    }

    fn base_of_code(&self) -> Option<u32> {
        (offset_of!(Header, base_of_code) + size_of_val(&self.header.base_of_code) <= self.size)
            .then_some(self.header.base_of_code)
    }

    fn image_base(&self) -> Option<u64> {
        (offset_of!(Header, image_base) + size_of_val(&self.header.image_base) <= self.size)
            .then_some(self.header.image_base)
    }

    fn section_alignment(&self) -> Option<u32> {
        (offset_of!(Header, section_alignment) + size_of_val(&self.header.section_alignment)
            <= self.size)
            .then_some(self.header.section_alignment)
    }

    fn file_alignment(&self) -> Option<u32> {
        (offset_of!(Header, file_alignment) + size_of_val(&self.header.file_alignment) <= self.size)
            .then_some(self.header.file_alignment)
    }

    fn major_operating_system_version(&self) -> Option<u16> {
        (offset_of!(Header, major_operating_system_version)
            + size_of_val(&self.header.major_operating_system_version)
            <= self.size)
            .then_some(self.header.major_operating_system_version)
    }

    fn minor_operating_system_version(&self) -> Option<u16> {
        (offset_of!(Header, minor_operating_system_version)
            + size_of_val(&self.header.minor_operating_system_version)
            <= self.size)
            .then_some(self.header.minor_operating_system_version)
    }

    fn major_image_version(&self) -> Option<u16> {
        (offset_of!(Header, major_image_version) + size_of_val(&self.header.major_image_version)
            <= self.size)
            .then_some(self.header.major_image_version)
    }

    fn minor_image_version(&self) -> Option<u16> {
        (offset_of!(Header, minor_image_version) + size_of_val(&self.header.minor_image_version)
            <= self.size)
            .then_some(self.header.minor_image_version)
    }

    fn major_subsystem_version(&self) -> Option<u16> {
        (offset_of!(Header, major_subsystem_version)
            + size_of_val(&self.header.major_subsystem_version)
            <= self.size)
            .then_some(self.header.major_subsystem_version)
    }

    fn minor_subsystem_version(&self) -> Option<u16> {
        (offset_of!(Header, minor_subsystem_version)
            + size_of_val(&self.header.minor_subsystem_version)
            <= self.size)
            .then_some(self.header.minor_subsystem_version)
    }

    fn win32_version_value(&self) -> Option<u32> {
        (offset_of!(Header, win32_version_value) + size_of_val(&self.header.win32_version_value)
            <= self.size)
            .then_some(self.header.win32_version_value)
    }

    fn size_of_image(&self) -> Option<u32> {
        (offset_of!(Header, size_of_image) + size_of_val(&self.header.size_of_image) <= self.size)
            .then_some(self.header.size_of_image)
    }

    fn size_of_headers(&self) -> Option<u32> {
        (offset_of!(Header, size_of_headers) + size_of_val(&self.header.size_of_headers)
            <= self.size)
            .then_some(self.header.size_of_headers)
    }

    fn checksum(&self) -> Option<u32> {
        (offset_of!(Header, checksum) + size_of_val(&self.header.checksum) <= self.size)
            .then_some(self.header.checksum)
    }

    fn subsystem(&self) -> Option<Subsystem> {
        (offset_of!(Header, subsystem) + size_of_val(&self.header.subsystem) <= self.size)
            .then(|| self.header.subsystem.clone())
    }

    fn dll_characteristics(&self) -> Option<u16> {
        (offset_of!(Header, dll_characteristics) + size_of_val(&self.header.dll_characteristics)
            <= self.size)
            .then_some(self.header.dll_characteristics)
    }

    fn size_of_stack_reserve(&self) -> Option<u64> {
        (offset_of!(Header, size_of_stack_reserve)
            + size_of_val(&self.header.size_of_stack_reserve)
            <= self.size)
            .then_some(self.header.size_of_stack_reserve)
    }

    fn size_of_stack_commit(&self) -> Option<u64> {
        (offset_of!(Header, size_of_stack_commit) + size_of_val(&self.header.size_of_stack_commit)
            <= self.size)
            .then_some(self.header.size_of_stack_commit)
    }

    fn size_of_heap_reserve(&self) -> Option<u64> {
        (offset_of!(Header, size_of_heap_reserve) + size_of_val(&self.header.size_of_heap_reserve)
            <= self.size)
            .then_some(self.header.size_of_heap_reserve)
    }

    fn size_of_heap_commit(&self) -> Option<u64> {
        (offset_of!(Header, size_of_heap_commit) + size_of_val(&self.header.size_of_heap_commit)
            <= self.size)
            .then_some(self.header.size_of_heap_commit)
    }

    fn loader_flags(&self) -> Option<u32> {
        (offset_of!(Header, loader_flags) + size_of_val(&self.header.loader_flags) <= self.size)
            .then_some(self.header.loader_flags)
    }

    fn number_of_rva_and_sizes(&self) -> Option<u32> {
        (offset_of!(Header, number_of_rva_and_sizes)
            + size_of_val(&self.header.number_of_rva_and_sizes)
            <= self.size)
            .then_some(self.header.number_of_rva_and_sizes)
    }

    fn data_directories(&'a self) -> Option<data::directories::WithSize<'a>> {
        (offset_of!(Header, data_directories) < self.size).then(|| {
            self.header
                .data_directories
                .with_size(self.size - offset_of!(Header, data_directories))
        })
    }
}

impl Debug for WithSize<'_> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        let mut debug_struct: DebugStruct = formatter.debug_struct("Header");
        if let Some(magic) = self.magic() {
            debug_struct.field("magic", &magic);
        }
        if let Some(major_linker_version) = self.major_linker_version() {
            debug_struct.field("major_linker_version", &major_linker_version);
        }
        if let Some(minor_linker_version) = self.minor_linker_version() {
            debug_struct.field("minor_linker_version", &minor_linker_version);
        }
        if let Some(size_of_code) = self.size_of_code() {
            debug_struct.field("size_of_code", &size_of_code);
        }
        if let Some(size_of_initialized_data) = self.size_of_initialized_data() {
            debug_struct.field("size_of_initialized_data", &size_of_initialized_data);
        }
        if let Some(size_of_uninitialized_data) = self.size_of_uninitialized_data() {
            debug_struct.field("size_of_uninitialized_data", &size_of_uninitialized_data);
        }
        if let Some(address_of_entry_point) = self.address_of_entry_point() {
            debug_struct.field("address_of_entry_point", &address_of_entry_point);
        }
        if let Some(base_of_code) = self.base_of_code() {
            debug_struct.field("base_of_code", &base_of_code);
        }
        if let Some(image_base) = self.image_base() {
            debug_struct.field("image_base", &image_base);
        }
        if let Some(section_alignment) = self.section_alignment() {
            debug_struct.field("section_alignment", &section_alignment);
        }
        if let Some(file_alignment) = self.file_alignment() {
            debug_struct.field("file_alignment", &file_alignment);
        }
        if let Some(major_operating_system_version) = self.major_operating_system_version() {
            debug_struct.field(
                "major_operating_system_version",
                &major_operating_system_version,
            );
        }
        if let Some(minor_operating_system_version) = self.minor_operating_system_version() {
            debug_struct.field(
                "minor_operating_system_version",
                &minor_operating_system_version,
            );
        }
        if let Some(major_image_version) = self.major_image_version() {
            debug_struct.field("major_image_version", &major_image_version);
        }
        if let Some(minor_image_version) = self.minor_image_version() {
            debug_struct.field("minor_image_version", &minor_image_version);
        }
        if let Some(major_subsystem_version) = self.major_subsystem_version() {
            debug_struct.field("major_subsystem_version", &major_subsystem_version);
        }
        if let Some(minor_subsystem_version) = self.minor_subsystem_version() {
            debug_struct.field("minor_subsystem_version", &minor_subsystem_version);
        }
        if let Some(win32_version_value) = self.win32_version_value() {
            debug_struct.field("win32_version_value", &win32_version_value);
        }
        if let Some(size_of_image) = self.size_of_image() {
            debug_struct.field("size_of_image", &size_of_image);
        }
        if let Some(size_of_headers) = self.size_of_headers() {
            debug_struct.field("size_of_headers", &size_of_headers);
        }
        if let Some(checksum) = self.checksum() {
            debug_struct.field("checksum", &checksum);
        }
        if let Some(subsystem) = self.subsystem() {
            debug_struct.field("subsystem", &subsystem);
        }
        if let Some(dll_characteristics) = self.dll_characteristics() {
            debug_struct.field("dll_characteristics", &dll_characteristics);
        }
        if let Some(size_of_stack_reserve) = self.size_of_stack_reserve() {
            debug_struct.field("size_of_stack_reserve", &size_of_stack_reserve);
        }
        if let Some(size_of_stack_commit) = self.size_of_stack_commit() {
            debug_struct.field("size_of_stack_commit", &size_of_stack_commit);
        }
        if let Some(size_of_heap_reserve) = self.size_of_heap_reserve() {
            debug_struct.field("size_of_heap_reserve", &size_of_heap_reserve);
        }
        if let Some(size_of_heap_commit) = self.size_of_heap_commit() {
            debug_struct.field("size_of_heap_commit", &size_of_heap_commit);
        }
        if let Some(loader_flags) = self.loader_flags() {
            debug_struct.field("loader_flags", &loader_flags);
        }
        if let Some(number_of_rva_and_sizes) = self.number_of_rva_and_sizes() {
            debug_struct.field("number_of_rva_and_sizes", &number_of_rva_and_sizes);
        }
        if let Some(data_directories) = self.data_directories() {
            debug_struct.field("data_directories", &data_directories);
        }
        debug_struct.finish()
    }
}
