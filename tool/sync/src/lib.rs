//! # References
//! * [Rust Atomics and Locks](https://www.oreilly.co.jp/books/9784814400515/)
//! # TODO
//! * Implement Arc after implementing memory allocator.
#![no_std]

extern crate alloc;

#[cfg(test)]
extern crate std;

pub mod arc;
pub mod oneshot;
pub mod spin;
