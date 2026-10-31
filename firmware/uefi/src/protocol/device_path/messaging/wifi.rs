use {
    super::{
        super::{Node, Type},
        SubType,
    },
    alloc::string::String,
    core::{
        fmt::{Debug, Formatter, Result},
        slice::from_raw_parts,
    },
};

/// # References
/// * [Wireless Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#wireless-device-path)
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
    ssid: [u8; 32],
}

impl Protocol {
    fn ssid_string(&self) -> String {
        self.ssid()
            .as_slice()
            .iter()
            .filter_map(u8::as_ascii)
            .collect()
    }
}

impl Debug for Protocol {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Protocol")
            .field("ty", &self.ty())
            .field("sub_type", &self.sub_type())
            .field("length", &self.length())
            .field("ssid", &self.ssid_string())
            .finish()
    }
}

impl Node for Protocol {}
