//! # References
//! * [SMBIOS Specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.10.0.pdf)
//! # TODO
//! * Implement SMBIOS tables.

#![no_std]

/// # References
/// * [SMBIOS Specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.10.0.pdf) 5.2.1 SMBIOS 2.1 (32-bit) Entry Point
#[derive(Debug)]
#[repr(C, packed)]
pub struct EntryPoint32 {
    anchor: [u8; 4],
}
