/// # References
/// * [Integer Baud Rate Register](https://support.arm.com/documentation/ddi0183/g/programmers-model/register-descriptions/integer-baud-rate-register--uartibrd?lang=en)
#[io::register]
pub struct Register {
    divisor: u16,
    __: u16,
}

impl Register {
    pub fn new(divisor: u16) -> Self {
        Self::default().update_divisor_u16(divisor)
    }
}
