/// # References
/// * [Data Register](https://support.arm.com/documentation/ddi0183/g/programmers-model/register-descriptions/data-register--uartdr?lang=en)
#[io::register]
pub struct Register {
    data: u8,
    framing_error: bool,
    parity_error: bool,
    break_error: bool,
    overrun_error: bool,
    __: [bool; 20],
}
