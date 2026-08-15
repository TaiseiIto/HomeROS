/// # References
/// * [Integer Baud Rate Register](https://support.arm.com/documentation/ddi0183/g/programmers-model/register-descriptions/integer-baud-rate-register--uartibrd?lang=en)
#[io::field]
pub struct Register {
    divisor: u16,
    __: u16,
}
