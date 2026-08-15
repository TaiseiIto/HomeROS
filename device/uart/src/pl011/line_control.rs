/// # References
/// * [Line Control Register](https://support.arm.com/documentation/ddi0183/g/programmers-model/register-descriptions/line-control-register--uartlcr-h?lang=en)
#[io::register]
pub struct Register {
    send_bread: bool,
    parity_enable: bool,
    even_parity: bool,
    two_stop_bits: bool,
    enable_fifos: bool,
    word_length: [bool; 2],
    stick_parity: bool,
    __: [u8; 3],
}
