/// # References
/// * [Data Register](https://support.arm.com/documentation/ddi0183/g/programmers-model/register-descriptions/data-register--uartdr?lang=en)
#[bit::field]
pub struct Register {
    data: [bool; 8],
    framing_error: bool,
    parity_error: bool,
    break_error: bool,
    overrun_error: bool,
    reserved: [bool; 20],
}
