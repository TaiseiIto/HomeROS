mod eoi;
mod id;
mod priority;
mod remote_read;
mod version;

/// # References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual](https://cdrdv2.intel.com/v1/dl/getContent/671200) 13.4.1 The Local APIC Block Diagram
/// # TODO
/// * Print all registers.
#[io::registers]
pub struct Registers {
    __: [u8; 0x20],
    id: id::Register,
    version: version::Register,
    __: [u8; 0x40],
    task_priority: priority::task::Register,
    arbitration_priority: priority::arbitration::Register,
    processor_priority: priority::processor::Register,
    eoi: eoi::Register,
    remote_read: remote_read::Register,
}
