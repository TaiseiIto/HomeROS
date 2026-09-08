#![feature(iter_array_chunks)]
#![no_std]

extern crate alloc;

mod header;
mod node;
mod property;
mod reserved_memory;
mod structure;

pub use {header::Header, node::Node};

use {alloc::vec::Vec, core::cell::OnceCell, memory::Regions, sync::spin::Lock};

pub static ROOT: Lock<OnceCell<Analyzed>> = Lock::new(OnceCell::new());

pub fn set(header: &Header) {
    ROOT.lock().set(header.into()).unwrap();
}

pub fn memory_regions() -> Regions<u128> {
    ROOT.lock().get().unwrap().memory_regions()
}

#[derive(Debug)]
pub struct Analyzed {
    root: node::Node,
    reserved_memory_entries: Vec<reserved_memory::Entry>,
}

impl Analyzed {
    fn memory_regions(&self) -> Regions<u128> {
        let Self {
            root,
            reserved_memory_entries,
        } = self;
        root.memories()
            .into_iter()
            .map(|node| node.regions())
            .sum::<Regions<u128>>()
            - (root
                .reserved_memories()
                .into_iter()
                .map(|node| node.regions())
                .sum::<Regions<u128>>()
                + reserved_memory_entries
                    .into_iter()
                    .map(|reserved_memory_entry| reserved_memory_entry.clone().into())
                    .sum::<Regions<u128>>())
    }
}

impl From<&Header> for Analyzed {
    fn from(header: &Header) -> Self {
        Self {
            root: header.root(),
            reserved_memory_entries: header.reserved_memory_entries(),
        }
    }
}
