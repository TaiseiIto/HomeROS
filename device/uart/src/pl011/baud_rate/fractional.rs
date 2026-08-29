/// # References
/// * [Fractional Baud Rate Register](https://support.arm.com/documentation/ddi0183/g/programmers-model/register-descriptions/fractional-baud-rate-register--uartfbrd?lang=en)
#[io::register]
pub struct Register {
    divisor: [bool; 6],
    __: [bool; 26],
}

impl Register {
    pub fn new(divisor: u32) -> Self {
        Self::default().update_divisor_shift(divisor)
    }
}
