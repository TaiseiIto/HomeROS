mod id;
mod version;

/// # References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual](https://cdrdv2.intel.com/v1/dl/getContent/671200) 13.4.1 The Local APIC Block Diagram
/// # TODO
/// * Print all registers.
#[io::registers]
pub struct Registers {
    __: [u128; 2],
    id: id::Register,
    version: version::Register,
}
