use {
    super::{
        super::{super::network::pxe::ip, Node, Type},
        SubType,
    },
    alloc::vec::Vec,
    core::{
        fmt::{Debug, Formatter, Result},
        marker::PhantomData,
        mem::{offset_of, size_of},
        slice::{from_raw_parts, from_raw_parts_mut},
    },
};

/// # References
/// * [DND Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#dns-device-path)
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    ty: Type,
    sub_type: SubType,
    length: u16,
    is_ipv6: u8,
    servers: PhantomData<ip::Address>,
}

impl Protocol {
    fn server_addresses(&self) -> Vec<ip::HeterogeneousAddress> {
        self.servers_bytes()
            .chunks(size_of::<ip::Address>())
            .map(|address| {
                let address: ip::Address =
                    unsafe { (address.as_ptr() as *const ip::Address).read_unaligned() };
                match self.is_ipv6() {
                    0x00 => ip::HeterogeneousAddress::V4(address.v4().clone()),
                    0x01 => ip::HeterogeneousAddress::V6(address.v6().clone()),
                    _ => panic!(),
                }
            })
            .collect()
    }

    fn servers_bytes(&self) -> &[u8] {
        unsafe {
            from_raw_parts(
                &self.servers as *const PhantomData<ip::Address> as *const u8,
                self.servers_size(),
            )
        }
    }

    fn servers_size(&self) -> usize {
        self.length() as usize - offset_of!(Self, servers)
    }
}

impl Debug for Protocol {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Protocol")
            .field("ty", &self.ty())
            .field("sub_type", &self.sub_type())
            .field("length", &self.length())
            .field("is_ipv6", &self.is_ipv6())
            .field("servers", &self.server_addresses())
            .finish()
    }
}

impl Node for Protocol {}
