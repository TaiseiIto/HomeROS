mod destination_format;
mod eoi;
mod id;
mod logical_destination;
mod priority;
mod remote_read;
mod spurious_interrupt_vector;
mod version;

/// # References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual](https://cdrdv2.intel.com/v1/dl/getContent/671200) 13.4.1 The Local APIC Block Diagram, Table 13-1. Local APIC Register Address Map
/// # TODO
/// * Print all registers.
#[io::registers]
pub struct Registers {
    __: [u8; 0x20],
    id: id::Register,
    __: [u8; 0xc],
    version: version::Register,
    __: [u8; 0x4c],
    task_priority: priority::task::Register,
    __: [u8; 0xc],
    arbitration_priority: priority::arbitration::Register,
    __: [u8; 0xc],
    processor_priority: priority::processor::Register,
    __: [u8; 0xc],
    eoi: eoi::Register,
    __: [u8; 0xc],
    remote_read: remote_read::Register,
    __: [u8; 0xc],
    logical_destination: logical_destination::Register,
    __: [u8; 0xc],
    destination_format: destination_format::Register,
    __: [u8; 0xc],
    spurious_interrupt_vector: spurious_interrupt_vector::Register,
    __: [u8; 0xc],
}
