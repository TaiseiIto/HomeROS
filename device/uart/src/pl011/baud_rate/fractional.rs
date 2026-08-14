/// # References
/// * [Fractional Baud Rate Register](https://support.arm.com/documentation/ddi0183/g/programmers-model/register-descriptions/fractional-baud-rate-register--uartfbrd?lang=en)
#[bit::field]
pub struct Register {
    divisor: [bool; 6],
    __: [bool; 26],
}
