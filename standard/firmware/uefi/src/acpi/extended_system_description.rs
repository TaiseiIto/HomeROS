use {
    super::Header,
    alloc::vec::Vec,
    core::fmt::{Debug, Formatter, Result},
};

/// # References
/// * [Extended System Description TAble (XSDT)](https://uefi.org/htmlspecs/ACPI_Spec_6_4_html/05_ACPI_Software_Programming_Model/ACPI_Software_Programming_Model.html#extended-system-description-table-xsdt)
#[repr(C, packed)]
pub struct Table {
    header: Header,
}

impl Table {
    fn entries(&self) -> Vec<&dyn super::Table> {
        self.header
            .body()
            .iter()
            .copied()
            .array_chunks()
            .map(|entry| {
                let header: usize = u64::from_le_bytes(entry) as usize;
                let header: *const Header = header as *const Header;
                unsafe { &*header }.table()
            })
            .collect()
    }

    fn entries_mut(&mut self) -> Vec<&mut dyn super::Table> {
        self.header
            .body()
            .iter()
            .copied()
            .array_chunks()
            .map(|entry| {
                let header: usize = u64::from_le_bytes(entry) as usize;
                let header: *mut Header = header as *mut Header;
                unsafe { &mut *header }.table_mut()
            })
            .collect()
    }

    fn initialize(&mut self) {
        for table in self.entries_mut().into_iter() {
            table.initialize();
        }
    }
}

impl Debug for Table {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Table")
            .field("header", &self.header)
            .field("entries", &self.entries())
            .finish()
    }
}

impl super::Table for Table {
    fn initialize(&mut self) {}
}
