use super::{
    super::{Node, Type},
    SubType,
};

/// # References
/// * [NVM Express namespace messaging device path node](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#nvm-express-namespace-messaging-device-path-node)
#[derive(Debug, packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
    identifier: u32,
    ieee_extended_unique_identifier: u64,
}

impl Node for Protocol {}
