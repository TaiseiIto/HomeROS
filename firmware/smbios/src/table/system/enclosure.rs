use {
    crate::{Header, Table as Trait},
    alloc::string::String,
    core::{
        fmt::{Debug, Formatter, Result},
        marker::PhantomData,
        slice::from_raw_parts,
    },
};

/// # References
/// * [SMBIOS Specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.10.0.pdf) 7.4 System Enclosure or Chassis (Type 3)
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Table {
    header: Header,
    manufacturer: u8,
    typ: u8,
    version: u8,
    serial_number: u8,
    asset_tag_number: u8,
    boot_up_state: u8,
    power_supply_state: u8,
    thermal_state: u8,
    security_status: u8,
    oem_defined: u32,
    height: u8,
    number_of_power_cords: u8,
    contained_element_count: u8,
    contained_element_record_length: u8,
    contained_elements: PhantomData<u8>,
}

impl Table {
    fn get_manufacturer(&self) -> Option<String> {
        self.manufacturer_index()
            .and_then(|index| self.strings().get(index).cloned())
    }

    fn get_version(&self) -> Option<String> {
        self.version_index()
            .and_then(|index| self.strings().get(index).cloned())
    }

    fn get_serial_number(&self) -> Option<String> {
        self.serial_number_index()
            .and_then(|index| self.strings().get(index).cloned())
    }

    fn get_asset_tag_number(&self) -> Option<String> {
        self.asset_tag_number_index()
            .and_then(|index| self.strings().get(index).cloned())
    }

    fn get_sku_number(&self) -> Option<String> {
        self.sku_number_index()
            .and_then(|index| self.strings().get(index).cloned())
    }

    fn manufacturer_index(&self) -> Option<usize> {
        (self.manufacturer() as usize).checked_sub(1)
    }

    fn version_index(&self) -> Option<usize> {
        (self.version() as usize).checked_sub(1)
    }

    fn serial_number_index(&self) -> Option<usize> {
        (self.serial_number() as usize).checked_sub(1)
    }

    fn asset_tag_number_index(&self) -> Option<usize> {
        (self.asset_tag_number() as usize).checked_sub(1)
    }

    fn sku_number_index(&self) -> Option<usize> {
        (self.footer().sku_number as usize).checked_sub(1)
    }

    fn get_contained_elements(&self) -> &[u8] {
        unsafe {
            from_raw_parts(
                &self.contained_elements as *const PhantomData<u8> as *const u8,
                (self.contained_element_count() * self.contained_element_record_length()) as usize,
            )
        }
    }

    fn footer(&self) -> &Footer {
        unsafe {
            &*(&self.contained_elements as *const PhantomData<u8> as *const Footer)
                .byte_add(self.get_contained_elements().len())
        }
    }
}

impl Debug for Table {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Table")
            .field("typ", &self.header.typ())
            .field("length", &self.header.length())
            .field("handle", &self.header.handle())
            .field("manufacturer", &self.get_manufacturer())
            .field("typ", &self.typ())
            .field("version", &self.get_version())
            .field("serial_number", &self.get_serial_number())
            .field("asset_tag_number", &self.get_asset_tag_number())
            .field("boot_up_state", &self.boot_up_state())
            .field("power_supply_state", &self.power_supply_state())
            .field("thermal_state", &self.thermal_state())
            .field("security_status", &self.security_status())
            .field("oem_defined", &self.oem_defined())
            .field("height", &self.height())
            .field("number_of_power_cords", &self.number_of_power_cords())
            .field("contained_element_count", &self.contained_element_count())
            .field(
                "contained_element_record_length",
                &self.contained_element_record_length(),
            )
            .field("contained_elements", &self.get_contained_elements())
            .field("sku_number", &self.get_sku_number())
            .field("rack_type", &self.footer().rack_type)
            .field("rack_height", &self.footer().rack_height)
            .finish()
    }
}

impl Trait for Table {}

#[derive(packed::Accessor, Debug)]
#[repr(C, packed)]
struct Footer {
    sku_number: u8,
    rack_type: u8,
    rack_height: u8,
}
