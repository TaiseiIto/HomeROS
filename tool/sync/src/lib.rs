//! # References
//! * [Rust Atomics and Locks](https://www.oreilly.co.jp/books/9784814400515/)
#![no_std]

extern crate alloc;

#[cfg(test)]
extern crate std;

pub mod arc;
pub mod oneshot;
pub mod spin;

pub use arc::{Arc, Weak};
