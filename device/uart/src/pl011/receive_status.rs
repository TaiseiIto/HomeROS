/// # References
/// * [Receive Status Register](https://support.arm.com/documentation/ddi0183/g/programmers-model/register-descriptions/receive-status-register---error-clear-register--uartrsr-uartecr?lang=en)
#[bit::field]
pub struct Register {
    framing_error: bool,
    parity_error: bool,
    break_error: bool,
    overrun_error: bool,
    reserved: [bool; 28],
}
