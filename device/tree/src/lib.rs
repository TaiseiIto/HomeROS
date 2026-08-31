#![feature(iter_array_chunks)]
#![no_std]

extern crate alloc;

mod header;
mod property;
mod structure;

pub use header::Header;
