use {
    super::{
        super::{Node, Type},
        SubType,
    },
    core::{
        fmt::{Debug, Formatter, Result},
        marker::PhantomData,
        mem::offset_of,
        slice::from_raw_parts,
    },
};

/// # References
/// * [NVMe over Fabric (NVME-oF) Namespace Device Path](https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html#nvme-over-fabric-nvme-of-namespace-device-path)
#[derive(packed::Accessor)]
#[repr(C, packed)]
pub struct Protocol {
    typ: u8,
    sub_type: u8,
    length: u16,
    nidt: u8,
    nid: u128,
    subsystem_nqn: PhantomData<u8>,
}

impl Protocol {
    fn get_type(&self) -> Type {
        self.typ().into()
    }

    fn get_sub_type(&self) -> SubType {
        self.sub_type().into()
    }

    fn subsystem_nqn_str(&self) -> &str {
        str::from_utf8(self.subsystem_nqn_bytes()).unwrap()
    }

    fn subsystem_nqn_bytes(&self) -> &[u8] {
        unsafe {
            from_raw_parts(
                &self.subsystem_nqn as *const PhantomData<u8> as *const u8,
                self.subsystem_nqn_size(),
            )
        }
    }

    fn subsystem_nqn_size(&self) -> usize {
        self.length() as usize - offset_of!(Self, subsystem_nqn)
    }
}

impl Debug for Protocol {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Protocol")
            .field("typ", &self.get_type())
            .field("sub_type", &self.get_sub_type())
            .field("length", &self.length())
            .field("nidt", &self.nidt())
            .field("nid", &self.nid())
            .field("subsystem_nqn", &self.subsystem_nqn_str())
            .finish()
    }
}

impl Node for Protocol {}
