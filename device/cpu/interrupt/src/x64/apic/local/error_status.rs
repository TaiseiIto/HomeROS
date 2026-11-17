/// # References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual](https://cdrdv2.intel.com/v1/dl/getContent/671200) 3.13.5.3 Error Handling, Figure 13-9. Error Status Register (ESR)
#[io::register]
pub struct Register {
    send_checksum_error: bool,
    receive_checksum_error: bool,
    send_accept_error: bool,
    receive_accept_error: bool,
    redirectable_ipi: bool,
    send_illegal_vector: bool,
    received_illegal_vector: bool,
    illegal_register_address: bool,
    __: [u8; 3],
}
