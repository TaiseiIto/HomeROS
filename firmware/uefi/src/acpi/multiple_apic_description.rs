use {
    alloc::vec::Vec,
    core::{
        fmt::{Debug, Formatter, Result},
        marker::PhantomData,
        mem::{offset_of, size_of},
    },
};

/// # References
/// * [Multiple APIC Description Table (MADT)](https://uefi.org/htmlspecs/ACPI_Spec_6_4_html/05_ACPI_Software_Programming_Model/ACPI_Software_Programming_Model.html#multiple-apic-description-table-madt)
/// # TODO
/// * Print interrupt controller structures.
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Table {
    header: super::Header,
    local_interrupt_controller_address: u32,
    flags: FlagsRaw,
    interrupt_controller_structure: PhantomData<u8>,
}

impl Table {
    fn get_flags(&self) -> Flags {
        self.flags().into()
    }

    fn get_interrupt_controller_structure_bytes(&self) -> &[u8] {
        &self.header.bytes()[offset_of!(Self, interrupt_controller_structure)..]
    }

    fn get_structures(&self) -> Vec<&dyn Structure> {
        let headers: Headers = self.get_interrupt_controller_structure_bytes().into();
        headers.map(Header::as_structure).collect()
    }
}

impl Debug for Table {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Table")
            .field("header", &self.header)
            .field(
                "local_interrupt_controller_address",
                &self.local_interrupt_controller_address(),
            )
            .field("flags", &self.get_flags())
            .field("interrupt_controller_structures", &self.get_structures())
            .finish()
    }
}

impl super::Table for Table {}

#[io::register]
struct Flags {
    pcat_compat: bool,
    __: [bool; 31],
}

#[derive(packed::Accessor)]
#[repr(C, packed)]
struct Header {
    typ: u8,
    length: u8,
}

impl Header {
    fn as_structure(&self) -> &dyn Structure {
        unimplemented!("type {:#x?}", self.get_type());
    }

    fn get_type(&self) -> Type {
        self.typ().into()
    }

    fn get_length(&self) -> usize {
        self.length() as usize
    }
}

impl Debug for Header {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Header")
            .field("typ", &self.get_type())
            .field("length", &self.get_length())
            .finish()
    }
}

/// # References
/// * [Interrupt Controller Structure Types](https://uefi.org/htmlspecs/ACPI_Spec_6_4_html/05_ACPI_Software_Programming_Model/ACPI_Software_Programming_Model.html#interrupt-controller-structure-types)
#[derive(Debug)]
enum Type {
    ProcessorLocalApic,
    IoApic,
    InterruptSourceOverride,
    NonMaskableInterruptSource,
    LocalApicNmi,
    LocalApicAddressOverrice,
    IoSapic,
    LocalSapic,
    PlatformInterruptSources,
    ProcessorLocalX2apic,
    LocalXapicNmi,
    GicCpuInterface,
    GicDistributor,
    GicMsiFrame,
    GicRedistributor,
    GicInterruptTranslationService,
    MultiprocessorWakeup,
    Reserved(u8),
}

impl From<u8> for Type {
    fn from(typ: u8) -> Self {
        match typ {
            0x00 => Self::ProcessorLocalApic,
            0x01 => Self::IoApic,
            0x02 => Self::InterruptSourceOverride,
            0x03 => Self::NonMaskableInterruptSource,
            0x04 => Self::LocalApicNmi,
            0x05 => Self::LocalApicAddressOverrice,
            0x06 => Self::IoSapic,
            0x07 => Self::LocalSapic,
            0x08 => Self::PlatformInterruptSources,
            0x09 => Self::ProcessorLocalX2apic,
            0x0a => Self::LocalXapicNmi,
            0x0b => Self::GicCpuInterface,
            0x0c => Self::GicDistributor,
            0x0d => Self::GicMsiFrame,
            0x0e => Self::GicRedistributor,
            0x0f => Self::GicInterruptTranslationService,
            0x10 => Self::MultiprocessorWakeup,
            typ => Self::Reserved(typ),
        }
    }
}

struct Headers<'a>(&'a [u8]);

impl<'a> From<&'a [u8]> for Headers<'a> {
    fn from(interrupt_controller_structure: &'a [u8]) -> Self {
        Self(interrupt_controller_structure)
    }
}

impl<'a> Iterator for Headers<'a> {
    type Item = &'a Header;

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .split_at_checked(size_of::<Self::Item>())
            .and_then(|(header, _)| header.first())
            .map(|header| unsafe { &*(header as *const u8 as *const Header) })
            .and_then(|header| {
                self.0
                    .split_at_checked(header.get_length())
                    .map(|(_, remaining)| {
                        self.0 = remaining;
                        header
                    })
            })
    }
}

trait Structure: Debug {}
