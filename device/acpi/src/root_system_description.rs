use {
    super::Header,
    alloc::vec::Vec,
    core::{
        fmt::{self, Debug, Formatter},
        mem::{offset_of, size_of, size_of_val},
        num::Wrapping,
        slice::from_raw_parts,
    },
};

/// # References
/// * [Root System Description Pointer (RSDP) Structure](https://uefi.org/htmlspecs/ACPI_Spec_6_4_html/05_ACPI_Software_Programming_Model/ACPI_Software_Programming_Model.html#rsdp-structure)
/// # TODO
/// * Print XSDT
#[repr(packed)]
pub struct Pointer {
    signature: [u8; 8],
    checksum: u8,
    oemid: [u8; 6],
    revision: u8,
    rsdt: [u8; 4],
    length: [u8; 4],
    xsdt: [u8; 8],
    extended_checksum: u8,
    __: [u8; 3],
}

impl Pointer {
    pub fn is_correct(&self) -> bool {
        let pointer: *const Self = self as *const Self;
        let pointer: *const u8 = pointer as *const u8;
        unsafe { from_raw_parts(pointer, offset_of!(Self, length)) }
            .iter()
            .copied()
            .map(Wrapping)
            .sum::<Wrapping<u8>>()
            .0
            == 0
            && unsafe { from_raw_parts(pointer, self.length()) }
                .iter()
                .copied()
                .map(Wrapping)
                .sum::<Wrapping<u8>>()
                .0
                == 0
    }

    fn length(&self) -> usize {
        u32::from_le_bytes(self.length.clone()) as usize
    }

    fn rsdt(&self) -> &Table {
        let rsdt: usize = u32::from_le_bytes(self.rsdt.clone()) as usize;
        let rsdt: *const Table = rsdt as *const Table;
        let rsdt: &Table = unsafe { &*rsdt };
        assert!(rsdt.header.is_correct());
        rsdt
    }

    fn signature(&self) -> &str {
        str::from_utf8(&self.signature).unwrap()
    }

    fn oemid(&self) -> &str {
        str::from_utf8(&self.oemid).unwrap()
    }
}

impl Debug for Pointer {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Pointer")
            .field("signature", &self.signature())
            .field("oemid", &self.oemid())
            .field("revision", &self.revision)
            .field("rsdt", &self.rsdt())
            .field("xsdt", &self.xsdt)
            .finish()
    }
}

/// # References
/// * [Root System Description TAble (RSDT)](https://uefi.org/htmlspecs/ACPI_Spec_6_4_html/05_ACPI_Software_Programming_Model/ACPI_Software_Programming_Model.html#root-system-description-table-rsdt)
/// # TODO
/// * Print entries
#[repr(packed)]
pub struct Table {
    header: Header,
}

impl Table {
    fn entries(&self) -> Vec<Result<&dyn super::Table, &str>> {
        self.header
            .body()
            .into_iter()
            .copied()
            .array_chunks()
            .map(|entry| {
                let header: usize = u32::from_le_bytes(entry) as usize;
                let header: *const Header = header as *const Header;
                unsafe { &*header }.table()
            })
            .collect()
    }
}

impl Debug for Table {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Table")
            .field("header", &self.header)
            .field("entries", &self.entries())
            .finish()
    }
}

impl super::Table for Table {}
