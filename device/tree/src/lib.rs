#![feature(iter_array_chunks)]
#![no_std]

extern crate alloc;

mod header;
mod node;
mod property;
mod reserved_memory;
mod structure;

pub use header::Header;
