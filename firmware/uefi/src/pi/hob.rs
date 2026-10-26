mod capsule;
mod cpu;
mod fv;
mod guid;
mod hand_off;
mod memory;
mod resource_descriptor;

use core::fmt::{self, Debug, Formatter};

#[repr(transparent)]
pub struct List(Header);

impl List {
    fn iter(&self) -> impl Iterator<Item = Result<&dyn Hob, &Type>> {
        HeaderIterator(&self.0).map(Header::hob)
    }
}

impl Debug for List {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.debug_list().entries(self.iter()).finish()
    }
}

struct HeaderIterator<'a>(&'a Header);

impl<'a> Iterator for HeaderIterator<'a> {
    type Item = &'a Header;

    fn next(&mut self) -> Option<Self::Item> {
        (self.0.ty != Type::EndOfHobList).then(|| {
            let item: Self::Item = self.0;
            self.0 = unsafe { &*(self.0 as *const Header).byte_add(item.length as usize) };
            item
        })
    }
}

/// # References
/// * [EFI_HOB_GENERIC_HEADER](https://uefi.org/specs/PI/1.10/V3_HOB_Code_Definitions.html#efi-hob-generic-header)
#[derive(Debug)]
#[repr(C)]
struct Header {
    ty: Type,
    length: u16,
    reserved: u32,
}

impl Header {
    fn hob(&self) -> Result<&dyn Hob, &Type> {
        match &self.ty {
            Type::Handoff => Ok(unsafe { &*(self as *const Header as *const hand_off::Hob) }),
            Type::MemoryAllocation => {
                Ok(unsafe { &*(self as *const Header as *const memory::allocation::Hob) })
            }
            Type::ResourceDescriptor => {
                Ok(unsafe { &*(self as *const Header as *const resource_descriptor::First) })
            }
            Type::ResourceDescriptor2 => {
                Ok(unsafe { &*(self as *const Header as *const resource_descriptor::Second) })
            }
            Type::Guid => Ok(unsafe { &*(self as *const Header as *const guid::Hob) }),
            Type::Fv => Ok(unsafe { &*(self as *const Header as *const fv::First) }),
            Type::Fv2 => Ok(unsafe { &*(self as *const Header as *const fv::Second) }),
            Type::Fv3 => Ok(unsafe { &*(self as *const Header as *const fv::Third) }),
            Type::Cpu => Ok(unsafe { &*(self as *const Header as *const cpu::Hob) }),
            Type::MemoryPool => {
                Ok(unsafe { &*(self as *const Header as *const memory::pool::Hob) })
            }
            Type::Capsule => Ok(unsafe { &*(self as *const Header as *const capsule::Hob) }),
            ty => Err(ty),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
#[repr(u16)]
enum Type {
    Handoff = 0x0001,
    MemoryAllocation = 0x0002,
    ResourceDescriptor = 0x0003,
    Guid = 0x0004,
    Fv = 0x0005,
    Cpu = 0x0006,
    MemoryPool = 0x0007,
    Fv2 = 0x0009,
    LoadPeimUnused = 0x000a,
    Capsule = 0x000b,
    Fv3 = 0x000c,
    ResourceDescriptor2 = 0x000d,
    Unused = 0xfffe,
    EndOfHobList = 0xffff,
}

trait Hob: Debug {}
